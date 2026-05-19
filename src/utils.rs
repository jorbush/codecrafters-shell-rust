use std::os::unix::fs::PermissionsExt;

pub fn is_executable(path: &str) -> bool {
    std::path::Path::new(path).exists()
        && std::fs::metadata(path).unwrap().permissions().mode() & 0o111 != 0
}

pub fn find_exec_dir(command: &str) -> Option<String> {
    let path = std::env::var("PATH").unwrap_or_default();
    let directories: Vec<&str> = path.split(':').collect();
    for dir in directories {
        let full_path = format!("{}/{}", dir, command);
        if is_executable(&full_path) {
            return Some(full_path);
        }
    }
    None
}
