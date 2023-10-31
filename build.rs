fn main() {
    // This script will run dx serve --hot-reload when building in release mode.
    if std::env::var("PROFILE").unwrap() == "release" {
        let status = std::process::Command::new("dx")
            .args(&["serve", "--hot-reload"])
            .status()
            .expect("Failed to start dx serve");
        assert!(status.success());
    }
}