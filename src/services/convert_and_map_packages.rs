use crate::models::package_info::PackageInfo;
use crate::services::fetch_package_details::fetch_package_details;

pub fn convert_and_map_packages(packages: Vec<(String, String)>) -> Vec<PackageInfo> {
    let mut package_infos = Vec::new();

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
                .unwrap_or_else(|| "unknown".to_string());
            let is_outdated = details.version != latest_version;
            let description = details
                .description
                .unwrap_or_else(|| "n/a".to_string());
            let project_url = details.project_url.unwrap_or_else(|| "n/a".to_string());
            let published_info = details.published_at
                .map(|d| d.format("%d %b %Y").to_string())
                .unwrap_or_else(|| "n/a".to_string());

            package_infos.push(PackageInfo {
                name,
                version,
                latest_version,
                published_date: published_info,
                license_url,
                license_expression,
                description,
                project_url,
                is_outdated,
                is_parsed_ok: true,
            });
        } else {
            package_infos.push(PackageInfo {
                name,
                version,
                latest_version: "unknown".to_string(),
                published_date: "n/a".to_string(),
                license_url: "not found".to_string(),
                license_expression: "not found".to_string(),
                description: "n/a".to_string(),
                project_url: "n/a".to_string(),
                is_outdated: false,
                is_parsed_ok: false,
            });
        }
    }

    package_infos
}