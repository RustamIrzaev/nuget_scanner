#[derive(Debug)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub latest_version: String,
    pub published_date: String,
    pub license_url: String,
    pub license_expression: String,
    pub description: String,
    pub project_url: String,
    pub is_outdated: bool,
    pub is_parsed_ok: bool,
}