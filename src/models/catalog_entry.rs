use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CatalogEntry {
    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "licenseUrl")]
    pub license_url: Option<String>,

    #[serde(rename = "licenseExpression")]
    pub license_expression: Option<String>,

    #[serde(rename = "projectUrl")]
    pub project_url: Option<String>,

    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(skip)]
    pub latest_version: Option<String>,

    #[serde(rename = "published")]
    pub published_at: Option<DateTime<Utc>>,
}