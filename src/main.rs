extern crate xml;

use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
struct Asset {
    path: String
}

impl Asset {
    fn new(path: String) -> Asset {
        Asset { path }
    }
}

#[derive(Debug)]
struct Bundle {
    name: String,
    size: u32,
    assets: Vec<Asset>
}

impl Bundle {
    fn new(name: String, size: u32) -> Bundle {
        Bundle { name, size, assets: vec![] }
    }
    fn add_asset(&mut self, asset: Asset) {
        self.assets.push(asset);
    }
}

#[derive(Debug)]
struct ResourceMap {
    bundles: Vec<Bundle>
}

impl ResourceMap {
    fn new() -> ResourceMap {
        ResourceMap{ bundles: vec![] }
    }
    fn add_bundle(&mut self, bundle: Bundle) {
        self.bundles.push(bundle);
    }
}

fn read_resource_map(xml: &str) -> Result<ResourceMap, &str> {
    let mut resource_map = ResourceMap::new();
    let mut current_bundle: Option<Bundle> = None;
    let parser = EventReader::new(xml.as_bytes());
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "Bundle" => {
                        let name = attributes.iter().find(|x| x.name.local_name.eq().as_str() == "Filename").unwrap().value.to_string();
                        let size = attributes.iter().find(|x| x.name.local_name.as_str() == "DownloadSize").unwrap().value.parse::<u32>().unwrap();
                        current_bundle = Some(Bundle::new(name, size));
                    },
                    "Asset" => {
                        let path_attr = attributes.iter().find(|x| x.name.local_name.as_str() == "AssetPath");//.unwrap().value.to_string();
                        match path_attr {
                            Some(path) => {
                                match current_bundle {
                                    Some(ref mut bundle) => bundle.add_asset(Asset::new(path.value.to_string())),
                                    None => return Err("Found opening Asset tag out of Bundle scope")
                                }
                            }
                            None => return Err("Cannot find attribute AssetPath in Asset tag")
                        }
                    },
                    _ => {}
                }
            },
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_str() {
                    "Bundle" => {
                        match current_bundle {
                            Some(_) => {
                                resource_map.add_bundle(current_bundle.unwrap());
                                current_bundle = None;
                            }
                            None => return Err("Found closing Bundle tag without opening one")
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
    Ok(resource_map)
}

fn main() {

    let xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<ResourceMapData>
  <Bundles>
    <Bundle Filename="bundle01" DownloadSize="42">
      <Asset AssetPath="assets/asset01" />
    </Bundle>
  </Bundles>
</ResourceMapData>
"#;

    let resource_map = read_resource_map(xml);
    println!("{:?}", resource_map);
}
