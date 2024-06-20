use walkdir::WalkDir;

pub fn find_package_files(folder: &str, max_depth: usize) -> Vec<String> {
    let mut package_files = Vec::new();

    for entry in WalkDir::new(folder)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "config" || ext == "csproj" {
                    package_files.push(path.to_string_lossy().to_string())
                }
            }
        }
    }

    package_files
}