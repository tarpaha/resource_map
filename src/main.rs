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

    let resource_map = resource_map::read_resource_map(xml.as_bytes());
    match resource_map {
        Ok(resource_map) => println!("{:?}", resource_map),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    bench_file_parsing();
}

fn bench_file_parsing() {
    use std::fs::File;
    use std::io::BufReader;
    use std::time::Instant;

    let file = File::open("resource_map.xml").unwrap();
    let file = BufReader::new(file);

    let now = Instant::now();
    let resource_map = resource_map::read_resource_map(file);
    match resource_map {
        Ok(resource_map) =>  {
            let elapsed = now.elapsed();
            let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
            println!("Bundles count: {}, seconds: {}", resource_map.get_bundles_count(), sec);
        },
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
