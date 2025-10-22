use std::process::Command;

// Save area
pub fn save(path: &str) {
    let geometry: String = get_geometry();
    save_area(&geometry, path);
}

// Get geometry from screen selection
fn get_geometry() -> String {
    let output = Command::new("slurp")
        .output()
        .expect("Failed to execute slurp");

    let geometry = str::from_utf8(&output.stdout).unwrap().trim();
    geometry.to_string()
}

// Save image based on geometry provided
fn save_area(geometry: &str, path: &str) {
    Command::new("grim")
        .arg("-g")
        .arg(geometry)
        .arg(path)
        .status()
        .expect("Failed to execute grim");
}
