use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};
use stremio_core::types::{
    addon::{
        Manifest, ManifestCatalog, ManifestExtra, ManifestResource, ResourceResponse, Version,
    },
    resource::{
        MetaItem, MetaItemBehaviorHints, MetaItemPreview, PosterShape, SeriesInfo, Stream,
        StreamSource, Video,
    },
};
use url::Url;

fn generate_manifest() {
    let url = Url::parse("https://imgs.search.brave.com/lXc3HFXtbr7MDZWknxiTleICFFrz7TcEFEQM1cd7j30/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9paDEu/cmVkYnViYmxlLm5l/dC9pbWFnZS41MDY5/MzczNTkuMDA5OS9m/bGF0LDc1MHgsMDc1/LGYtcGFkLDc1MHgx/MDAwLGY4ZjhmOC51/My5qcGc").unwrap();
    let manifest = Manifest {
        id: "com.illusionaryfrog.msa".to_owned(),
        version: Version::new(1, 1, 1),
        name: "MSA".to_owned(),
        contact_email: Some("mail@illusionaryfrog.com".to_owned()),
        description: Some("My Stremio Addon".to_owned()),
        logo: Some(url.clone()),
        background: Some(url),
        types: vec!["movie".to_owned(), "series".to_owned()],
        resources: vec![
            ManifestResource::Short("catalog".to_owned()),
            ManifestResource::Full {
                name: "meta".to_owned(),
                id_prefixes: Some(vec!["msa-".to_owned()]),
                types: Some(vec!["movie".to_owned(), "series".to_owned()]),
            },
            ManifestResource::Full {
                name: "stream".to_owned(),
                id_prefixes: Some(vec!["msa-".to_owned()]),
                types: Some(vec!["movie".to_owned(), "series".to_owned()]),
            },
        ],
        id_prefixes: None,
        catalogs: vec![ManifestCatalog {
            id: "msa-catalog".to_owned(),
            r#type: "MSA-Catalog".to_owned(),
            name: Some("Media".to_owned()),
            extra: ManifestExtra::Short {
                required: vec![],
                supported: vec![],
            },
        }],
        addon_catalogs: vec![],
        behavior_hints: Default::default(),
    };
    let out = serde_json::to_string(&manifest).unwrap();
    fs::write("./docs/manifest.json", &out).unwrap();
}

fn generate_resources() {
    let bpath = Path::new("/media");

    #[derive(Deserialize)]
    struct Meta {
        r#type: String,
        name: String,
        image: String,
        description: String,
        folder: String,
    }

    let mbytes = fs::read(bpath.join("meta.json")).unwrap();
    let metas: HashMap<usize, Meta> = serde_json::from_slice(&mbytes).unwrap();

    let metas: Vec<(usize, Meta, MetaItemPreview)> = metas
        .into_iter()
        .map(|(id, meta)| {
            let name = meta.name.to_owned();
            let r#type = meta.r#type.to_owned();
            let description = meta.description.to_owned();
            let image = Url::parse(&meta.image).unwrap();
            (
                id,
                meta,
                MetaItemPreview {
                    id: format!("msa-{:04x}{:04x}{:04x}", id, 1, 1),
                    r#type: r#type,
                    name: name,
                    poster: Some(image.clone()),
                    background: Some(image.clone()),
                    logo: Some(image),
                    description: Some(description),
                    release_info: None,
                    runtime: None,
                    released: None,
                    poster_shape: PosterShape::default(),
                    links: vec![],
                    trailer_streams: vec![],
                    behavior_hints: MetaItemBehaviorHints::default(),
                },
            )
        })
        .collect();

    let catalog = ResourceResponse::Metas {
        metas: metas
            .iter()
            .map(|(_, _, preview)| preview.clone())
            .collect(),
    };

    let out = serde_json::to_string(&catalog).unwrap();
    fs::write("./docs/catalog/MSA-Catalog/msa-catalog.json", &out).unwrap();

    #[derive(Deserialize)]
    struct SMeta {
        title: String,
        file: String,
    }

    for (id, meta, preview) in metas.into_iter() {
        let spath = bpath.join(&meta.folder);
        let smeta = fs::read(spath.join("meta.json")).unwrap();
        let smetas: HashMap<usize, HashMap<usize, SMeta>> = serde_json::from_slice(&smeta).unwrap();

        let mut videos = vec![];
        for (s, smetas) in smetas.into_iter() {
            for (e, smeta) in smetas.into_iter() {
                let sid = format!("msa-{:04x}{:04x}{:04x}", id, s, e);
                let url = format!(
                    "http://192.168.178.20:29839{}",
                    spath.join(smeta.file).to_str().unwrap()
                );
                let stream = Stream {
                    name: None,
                    description: None,
                    thumbnail: Some(meta.image.clone()),
                    source: StreamSource::Url {
                        url: Url::parse(&url).unwrap(),
                    },
                    subtitles: vec![],
                    behavior_hints: Default::default(),
                };

                let streams = ResourceResponse::Streams {
                    streams: vec![stream.clone()],
                };

                let out = serde_json::to_string(&streams).unwrap();
                let opath = format!("./docs/stream/{}/{sid}.json", meta.r#type);
                fs::write(opath, &out).unwrap();

                if meta.r#type == "series" {
                    videos.push(Video {
                        id: sid,
                        title: smeta.title,
                        thumbnail: Some(meta.image.clone()),
                        streams: vec![stream],
                        trailer_streams: vec![],
                        overview: None,
                        released: None,
                        series_info: Some(SeriesInfo {
                            season: s as u32,
                            episode: e as u32,
                        }),
                    })
                }
            }
        }

        let resource = ResourceResponse::Meta {
            meta: MetaItem {
                preview: preview,
                videos: videos,
            },
        };

        let out = serde_json::to_string(&resource).unwrap();
        let opath = format!(
            "./docs/meta/{}/msa-{:04x}{:04x}{:04x}.json",
            meta.r#type, id, 1, 1
        );
        fs::write(opath, &out).unwrap();
    }
}

fn main() {
    generate_manifest();
    generate_resources();
}
