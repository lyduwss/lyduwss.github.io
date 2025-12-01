use std::time::SystemTime;

fn main() {
    // Generate build timestamp as version
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);
}
