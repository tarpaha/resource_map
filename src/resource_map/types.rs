#[derive(Debug)]
pub struct Asset {
    path: String
}

impl Asset {
    pub fn new(path: String) -> Asset {
        Asset { path }
    }
}

#[derive(Debug)]
pub struct Bundle {
    name: String,
    size: u32,
    assets: Vec<Asset>
}

impl Bundle {
    pub fn new(name: String, size: u32) -> Bundle {
        Bundle { name, size, assets: vec![] }
    }
    pub fn add_asset(&mut self, asset: Asset) {
        self.assets.push(asset);
    }
}

#[derive(Debug)]
pub struct ResourceMap {
    bundles: Vec<Bundle>
}

impl ResourceMap {
    pub fn new() -> ResourceMap {
        ResourceMap{ bundles: vec![] }
    }
    pub fn add_bundle(&mut self, bundle: Bundle) {
        self.bundles.push(bundle);
    }
}
