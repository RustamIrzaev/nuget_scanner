use super::package_info::PackageInfo;

pub struct ProjectInfo {
    pub project_name: String,
    pub packages: Vec<PackageInfo>,
}