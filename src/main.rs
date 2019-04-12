use std::process;

mod resource_map;

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

    let resource_map = resource_map::read_resource_map(xml);
    match resource_map {
        Ok(resource_map) => println!("{:?}", resource_map),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
