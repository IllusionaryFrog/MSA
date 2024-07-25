use actix_files::Files;
use actix_web::{
    dev::Service,
    get,
    http::header::{HeaderValue, ACCESS_CONTROL_ALLOW_ORIGIN, CACHE_CONTROL},
    web, App, HttpServer, Responder, Result,
};
use futures_util::future::FutureExt;
use serde::Deserialize;
use std::fs;
use std::{collections::HashMap, path::Path, str::FromStr};
use stremio_core::types::{
    addon::{
        ExtraProp, Manifest, ManifestCatalog, ManifestExtra, ManifestResource, ResourceResponse,
        Version,
    },
    resource::{
        MetaItem, MetaItemBehaviorHints, MetaItemPreview, PosterShape, SeriesInfo, Stream,
        StreamSource, Video,
    },
};
use url::Url;

#[get("/manifest.json")]
async fn manifest() -> Result<impl Responder> {
    let url = url::Url::from_str("https://imgs.search.brave.com/lXc3HFXtbr7MDZWknxiTleICFFrz7TcEFEQM1cd7j30/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9paDEu/cmVkYnViYmxlLm5l/dC9pbWFnZS41MDY5/MzczNTkuMDA5OS9m/bGF0LDc1MHgsMDc1/LGYtcGFkLDc1MHgx/MDAwLGY4ZjhmOC51/My5qcGc").unwrap();
    Ok(web::Json(Manifest {
        id: "com.lukashassler.msa".to_owned(),
        version: Version::new(1, 0, 0),
        name: "MSA".to_owned(),
        contact_email:  Some("mail@lukashassler.com".to_owned()),
        description: Some("My Stremio Addon".to_owned()),
        logo: Some(url.clone()),
        background: Some(url),
        types: vec!["movie".to_owned(), "series".to_owned()],
        resources: vec![
            ManifestResource::Short("catalog".to_owned()),
            ManifestResource::Short("meta".to_owned()),
            ManifestResource::Short("stream".to_owned()),
        ],
        id_prefixes: Some(vec!["msa:".to_owned()]),
        catalogs: vec![ManifestCatalog {
            id: "msa:catalog".to_owned(),
            r#type: "MSA-Catalog".to_owned(),
            name: Some("Media".to_owned()),
            extra: ManifestExtra::Full { props: vec![ExtraProp {
                name: "search".to_owned(),
                is_required: false,
                options: vec![],
                options_limit: Default::default(),
            }] },
        }],
        addon_catalogs: vec![],
        behavior_hints: Default::default(),
    }))
}

#[get("/{resource}/{type}/msa:{id}.json")]
async fn handle(path: web::Path<(Resource, String, String)>) -> Result<impl Responder> {
    let (r, t, id) = path.into_inner();
    Ok(web::Json(_handle(r, t, id, String::new())))
}

#[get("/{resource}/{type}/msa:{id}/{extraArgs}.json")]
async fn handle_extra(
    path: web::Path<(Resource, String, String, String)>,
) -> Result<impl Responder> {
    let (r, t, id, e) = path.into_inner();
    Ok(web::Json(_handle(r, t, id, e)))
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum Resource {
    Catalog,
    Meta,
    Stream,
}

fn _handle(r: Resource, _: String, id: String, _e: String) -> ResourceResponse {
    let bpath = Path::new("/media");

    #[derive(Deserialize)]
    struct Meta {
        r#type: String,
        name: String,
        image: String,
        description: String,
        folder: String,
    }

    #[derive(Deserialize)]
    struct SMeta {
        title: String,
        file: String,
    }

    let meta = fs::read(bpath.join("meta.json")).unwrap();
    let metas: HashMap<usize, Meta> = serde_json::from_slice(&meta).unwrap();

    match r {
        Resource::Catalog => {
            ResourceResponse::Metas {
                metas: metas
                    .into_iter()
                    .filter_map(|(id, meta)| {
                        let image = Url::parse(&meta.image).unwrap();
                        // TODO: implement search filter
                        Some(MetaItemPreview {
                            id: format!("msa:{:03}0101", id),
                            r#type: meta.r#type,
                            name: meta.name,
                            poster: Some(image.clone()),
                            background: Some(image.clone()),
                            logo: Some(image),
                            description: Some(meta.description),
                            release_info: None,
                            runtime: None,
                            released: None,
                            poster_shape: PosterShape::default(),
                            links: vec![],
                            trailer_streams: vec![],
                            behavior_hints: MetaItemBehaviorHints::default(),
                        })
                    })
                    .collect(),
            }
        }
        Resource::Meta => {
            let id = id.parse::<usize>().unwrap() / 10000;

            let meta = metas.get(&id).unwrap();
            let videos = if meta.r#type == "series" {
                let spath = bpath.join(&meta.folder);
                let smeta = fs::read(spath.join("meta.json")).unwrap();
                let smetas: HashMap<usize, HashMap<usize, SMeta>> =
                    serde_json::from_slice(&smeta).unwrap();

                smetas
                    .into_iter()
                    .flat_map(|(s, smeta)| {
                        let spath = spath.clone();
                        smeta.into_iter().map(move |(e, smeta)| {
                            let url = format!(
                                "http://192.168.178.20:8080{}",
                                spath.join(smeta.file).to_str().unwrap()
                            );
                            Video {
                                id: format!("msa:{:03}{:02}{:02}", id, s, e),
                                title: smeta.title,
                                thumbnail: Some(meta.image.clone()),
                                streams: vec![Stream {
                                    name: None,
                                    description: None,
                                    thumbnail: Some(meta.image.clone()),
                                    source: StreamSource::Url {
                                        url: Url::parse(&url).unwrap(),
                                    },
                                    subtitles: vec![],
                                    behavior_hints: Default::default(),
                                }],
                                trailer_streams: vec![],
                                overview: None,
                                released: None,
                                series_info: Some(SeriesInfo {
                                    season: s as u32,
                                    episode: e as u32,
                                }),
                            }
                        })
                    })
                    .collect()
            } else {
                vec![]
            };

            let image = Url::parse(&meta.image).unwrap();
            ResourceResponse::Meta {
                meta: MetaItem {
                    preview: MetaItemPreview {
                        id: format!("msa:{}", id),
                        r#type: meta.r#type.clone(),
                        name: meta.name.clone(),
                        poster: Some(image.clone()),
                        background: Some(image.clone()),
                        logo: Some(image),
                        description: Some(meta.description.clone()),
                        release_info: None,
                        runtime: None,
                        released: None,
                        poster_shape: PosterShape::default(),
                        links: vec![],
                        trailer_streams: vec![],
                        behavior_hints: MetaItemBehaviorHints::default(),
                    },
                    videos: videos,
                },
            }
        }
        Resource::Stream => {
            let id = id.parse::<usize>().unwrap();

            let mid = id / 10000;
            let sid = id % 10000;
            let eid = sid % 100;
            let sid = sid / 100;

            let meta = metas.get(&mid).unwrap();

            let spath = bpath.join(&meta.folder);
            let smeta = fs::read(spath.join("meta.json")).unwrap();
            let smetas: HashMap<usize, HashMap<usize, SMeta>> =
                serde_json::from_slice(&smeta).unwrap();

            let smeta = smetas.get(&sid).unwrap().get(&eid).unwrap();
            let url = format!(
                "http://192.168.178.20:8080{}",
                spath.join(&smeta.file).to_str().unwrap()
            );
            ResourceResponse::Streams {
                streams: vec![Stream {
                    name: None,
                    description: None,
                    thumbnail: Some(meta.image.clone()),
                    source: StreamSource::Url {
                        url: Url::parse(&url).unwrap(),
                    },
                    subtitles: vec![],
                    behavior_hints: Default::default(),
                }],
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                srv.call(req).map(|res| {
                    res.map(|mut res| {
                        res.headers_mut()
                            .insert(CACHE_CONTROL, HeaderValue::from_static("public, max-age=5"));
                        res.headers_mut()
                            .insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
                        res
                    })
                })
            })
            .service(Files::new("/media", "/media").show_files_listing())
            .service(manifest)
            .service(handle)
            .service(handle_extra)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
