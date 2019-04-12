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

    let rm = resource_map::read_resource_map(xml);
    println!("{:?}", rm);
}
