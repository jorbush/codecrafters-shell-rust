use std::os::unix::fs::PermissionsExt;

pub fn is_executable(path: &str) -> bool {
    std::path::Path::new(path).exists()
        && std::fs::metadata(path).unwrap().permissions().mode() & 0o111 != 0
}
