use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn parse_packages_config(file_path: &str) -> Vec<(String, String)> {
    let regex = Regex::new(r#"<package id="([^"]+)" version="([^"]+)""#).unwrap();
    
    extract_packages_info_from_file(file_path, regex)
}

pub fn parse_csproj(file_path: &str) -> Vec<(String, String)> {
    let regex = Regex::new(r#"<PackageReference Include="([^"]+)" Version="([^"]+)""#).unwrap();

    extract_packages_info_from_file(file_path, regex)
}

fn extract_packages_info_from_file(file_path: &str, regex: Regex) -> Vec<(String, String)> {
    let mut packages = Vec::new();
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(captures) = regex.captures(&line) {
                packages.push((captures[1].to_string(), captures[2].to_string()));
            }
        }
    }

    packages
}