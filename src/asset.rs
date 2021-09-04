use std::fs;

pub fn get_asset_path(path: &str) -> String {
    let static_dir: &str = "static";
    let manifest_file_name: &str = "assets-manifest.json";

    let manifest = format!("{}/{}", static_dir, manifest_file_name);

    let manifest_content = match fs::read_to_string(manifest) {
        Ok(manifest_content) => manifest_content,
        Err(_error) => panic!("Impossible to read manifest file."),
    };

    let json: serde_json::Value = match serde_json::from_str(manifest_content.as_str()) {
        Ok(json) => json,
        Err(_error) => panic!("Impossible to parse manifest file."),
    };

    let asset_path = match json.get(path) {
        Some(asset_path) => asset_path.as_str().unwrap(),
        None => path,
    };

    let public_path = format!("/{}/{}", static_dir, asset_path);
    public_path
}