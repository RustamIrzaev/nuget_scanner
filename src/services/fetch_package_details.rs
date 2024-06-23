use std::io::Read;
use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use semver::Version;
use serde::Deserialize;
use crate::models::catalog_entry::CatalogEntry;

#[derive(Deserialize)]
struct PackageVersion {
    #[serde(rename = "catalogEntry")]
    catalog_entry: CatalogEntry,
}

#[derive(Deserialize)]
struct RegistrationPage {
    items: Vec<PackageVersion>,
}

#[derive(Deserialize)]
struct RegistrationIndex {
    items: Vec<RegistrationPage>,
}

pub fn fetch_package_details(package_name: &str, version: &str) -> Option<CatalogEntry> {
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
        Err(_) => {
            // eprintln!("Failed to deserialize JSON: {}", e);
            None
        }
    }
}