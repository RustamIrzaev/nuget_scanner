use clap::{arg, Parser};
use colored::Colorize;
use flate2::read::GzDecoder;
use regex::Regex;
use reqwest::blocking::Client;
use semver::Version;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use walkdir::WalkDir;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    folder: PathBuf,

    #[clap(short, long, default_value = "10")]
    max_depth: usize,
}

#[derive(Deserialize)]
struct PackageVersion {
    #[serde(rename = "catalogEntry")]
    catalog_entry: CatalogEntry,
}

#[derive(Deserialize, Clone)]
struct CatalogEntry {
    #[serde(rename = "version")]
    version: String,

    #[serde(rename = "licenseUrl")]
    license_url: Option<String>,

    #[serde(rename = "licenseExpression")]
    license_expression: Option<String>,

    #[serde(rename = "projectUrl")]
    project_url: Option<String>,

    #[serde(rename = "description")]
    description: Option<String>,

    #[serde(skip)]
    latest_version: Option<String>,

    #[serde(rename = "published")]
    published_at: Option<DateTime<Utc>>,
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
    let args = Cli::parse();

    let folder = args.folder.to_string_lossy().to_string();
    let package_files = find_package_files(&folder, args.max_depth);

    let mut packages = Vec::new();

    for file in package_files {
        if file.ends_with("packages.config") {
            packages.extend(parse_packages_config(&file));
        } else if file.ends_with(".csproj") {
            packages.extend(parse_csproj(&file));
        }
    }

    if packages.is_empty() {
        eprintln!("No NuGet packages found in the specified folder");
        return;
    }

    for (name, version) in packages {
        if let Some(details) = fetch_package_details(&name, &version) {
            let license_url = details
                .license_url
                .unwrap_or_else(|| "not found".to_string());
            let license_expression = details
                .license_expression
                .unwrap_or_else(|| "not found".to_string());
            let latest_version = details
                .latest_version
                .clone()
                .unwrap_or_else(|| "Unknown".to_string());
            let is_outdated = details.version != latest_version;
            let description = details
                .description
                .unwrap_or_else(|| "not found".to_string());
            let project_url = details.project_url.unwrap_or_else(|| "n/a".to_string());
            let published_info = details.published_at
                .map(|d| d.format("%d %b %Y").to_string())
                .unwrap_or_else(|| "n/a".to_string());

            println!(
                "- {}, version {} {} {}",
                name.bold().bright_blue(),
                version.italic().bright_blue(),
                if is_outdated {
                    "[outdated]".bright_red()
                } else {
                    "".normal()
                },
                if is_outdated {
                    format!(" latest: {}", latest_version.bright_green())
                } else {
                    "".to_string()
                }
            );

            println!("  license: {}", license_expression.bright_yellow());
            println!("  license URL: {}", license_url.bright_purple());
            println!("  description: {}", description.bright_cyan());
            println!("  package URL: {}", project_url.bright_magenta());
            println!("  published at: {}", published_info.bright_cyan());
            println!();
        } else {
            println!(
                "- {}, version {}",
                name.bold().bright_blue(),
                version.italic().bright_blue()
            );
            println!(
                "  {}",
                "Unable to fetch package details".italic().red().to_string()
            );
            println!();
        }
    }
}

fn find_package_files(folder: &str, max_depth: usize) -> Vec<String> {
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
    let url = format!(
        "https://api.nuget.org/v3/registration5-gz-semver1/{}/index.json",
        package_name.to_lowercase()
    );
    let client = Client::new();
    let response = client
        .get(&url)
        .header("Accept-Encoding", "gzip")
        .send()
        .ok()?;

    let mut decoder = GzDecoder::new(response);
    let mut body = String::new();

    if decoder.read_to_string(&mut body).is_err() {
        return None;
    }
    // println!("Response: {}", body);

    match serde_json::from_str::<RegistrationIndex>(&body) {
        Ok(registration_index) => {
            let mut latest_version: Option<Version> = None;
            let mut latest_entry: Option<&CatalogEntry> = None;
            let mut target_entry: Option<&CatalogEntry> = None;

            for page in &registration_index.items {
                for package_version in &page.items {
                    let entry = &package_version.catalog_entry;

                    if let Ok(entry_version) = Version::parse(&entry.version) {
                        if latest_version.is_none()
                            || entry_version > *latest_version.as_ref().unwrap()
                        {
                            latest_version = Some(entry_version.clone());
                            latest_entry = Some(entry);
                        }

                        if entry.version == version {
                            target_entry = Some(entry);
                        }
                    }
                }
            }

            if let Some(target) = target_entry {
                let mut entry_with_latest = target.clone();

                if let Some(latest) = latest_entry {
                    entry_with_latest.latest_version = Some(latest.version.clone());
                }

                return Some(entry_with_latest);
            }

            None
        }
        Err(e) => {
            eprintln!("Failed to deserialize JSON: {}", e);
            None
        }
    }
}
