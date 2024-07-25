use std::fs;
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

fn manifest() {
    let url = Url::parse("https://imgs.search.brave.com/lXc3HFXtbr7MDZWknxiTleICFFrz7TcEFEQM1cd7j30/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9paDEu/cmVkYnViYmxlLm5l/dC9pbWFnZS41MDY5/MzczNTkuMDA5OS9m/bGF0LDc1MHgsMDc1/LGYtcGFkLDc1MHgx/MDAwLGY4ZjhmOC51/My5qcGc").unwrap();
    let manifest = Manifest {
        id: "com.illusionaryfrog.msa".to_owned(),
        version: Version::new(1, 0, 0),
        name: "MSA".to_owned(),
        contact_email:  Some("mail@illusionaryfrog.com".to_owned()),
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
    };
    let out = serde_json::to_string(&manifest).unwrap();
    fs::write("./docs/manifest.json", &out).unwrap();
}

fn main() {
    manifest();
}
