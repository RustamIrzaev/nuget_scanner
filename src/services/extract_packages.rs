use std::fs;
use std::path::{Path, PathBuf};
use crate::utils::parse_configs::{parse_csproj, parse_packages_config};

pub fn extract_packages(project_path: &Path) -> Vec<(String, String)> {
    let mut packages = Vec::new();
    let csproj_files: Vec<PathBuf> = fs::read_dir(project_path)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.extension().map(|ext| ext == "csproj").unwrap_or(false) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    for csproj_file in &csproj_files {
        let packages_config_path = csproj_file.with_file_name("packages.config");

        if packages_config_path.exists() {
            packages.extend(parse_packages_config(packages_config_path.to_str().unwrap()));
        } else {
            packages.extend(parse_csproj(csproj_file.to_str().unwrap()));
        }
    }

    packages
}