use std::path::PathBuf;

/// Get path to the Rust project root directory
pub fn get_project_root() -> PathBuf {
    let project_path = std::env::var("CARGO_MANIFEST_DIR")
        .expect("Environment variable CARGO_MANIFEST_DIR is not defined");

    PathBuf::from(project_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_project_root() {
        assert!(get_project_root().ends_with("common"));
    }
}
