use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use flate2::read::GzDecoder;
use regex::Regex;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct PackageVersion {
    #[serde(rename = "catalogEntry")]
    catalog_entry: CatalogEntry,
}

#[derive(Deserialize)]
struct CatalogEntry {
    #[serde(rename = "version")]
    version: String,
    #[serde(rename = "licenseUrl")]
    license_url: Option<String>,
    #[serde(rename = "licenseExpression")]
    license_expression: Option<String>,
}

#[derive(Deserialize)]
struct RegistrationPage {
    items: Vec<PackageVersion>,
}

#[derive(Deserialize)]
struct RegistrationIndex {
    items: Vec<RegistrationPage>,
}

fn main() {
    let dir = "<Path_to_c#_project>";
    let package_files = find_package_files(dir);
    
    let mut packages = Vec::new();
    
    for file in package_files {
        if file.ends_with("packages.config") {
            packages.extend(parse_packages_config(&file));
        } else if file.ends_with(".csproj") {
            packages.extend(parse_csproj(&file));
        }
    }

    for (name, version) in packages {
        if let Some(details) = fetch_package_details(&name, &version) {
            let license_url = details.license_url.unwrap_or_else(|| "License URL not found".to_string());
            let license_expression = details.license_expression.unwrap_or_else(|| "License expression not found".to_string());
            
            println!("Package: {}, Version: {}, License URL: {}, License Expression: {}", name, version, license_url, license_expression);
        } else {
            println!("Package: {}, Version: {}, License: Not Found", name, version);
        }
    }
}

fn find_package_files(dir: &str) -> Vec<String> {
    let mut package_files = Vec::new();
    
    for entry in fs::read_dir(dir)
        .expect("Unable to read directory") {
        let entry = entry.expect("Unable to read entry");
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

fn parse_packages_config(file_path: &str) -> Vec<(String, String)> {
    let mut packages = Vec::new();
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let regex = Regex::new(r#"<package id="([^"]+)" version="([^"]+)""#).unwrap();
    
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(captures) = regex.captures(&line) {
                packages.push((captures[1].to_string(), captures[2].to_string()));
            }
        }
    }
    
    packages
}

fn parse_csproj(file_path: &str) -> Vec<(String, String)> {
    let mut packages = Vec::new();
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let regex = Regex::new(r#"<PackageReference Include="([^"]+)" Version="([^"]+)""#).unwrap();

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(captures) = regex.captures(&line) {
                packages.push((captures[1].to_string(), captures[2].to_string()));
            }
        }
    }
    
    packages
}

fn fetch_package_details(package_name: &str, version: &str) -> Option<CatalogEntry> {
    let url = format!("https://api.nuget.org/v3/registration5-gz-semver2/{}/index.json", package_name.to_lowercase());
    let client = Client::new();
    let response = client.get(&url).header("Accept-Encoding", "gzip").send().ok()?;

    let mut decoder = GzDecoder::new(response);
    let mut body = String::new();
    
    if decoder.read_to_string(&mut body).is_err() {
        return None;
    }
    // println!("Response: {}", body);

    match serde_json::from_str::<RegistrationIndex>(&body) {
        Ok(registration_index) => {
            for page in registration_index.items {
                for package_version in page.items {
                    if package_version.catalog_entry.version == version {
                        return Some(package_version.catalog_entry);
                    }
                }
            }
            None
        },
        Err(e) => {
            eprintln!("Failed to deserialize JSON: {}", e);
            None
        }
    }
}