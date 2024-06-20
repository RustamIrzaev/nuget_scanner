use clap::Parser;
use colored::Colorize;

use crate::cli::Cli;
use crate::services::convert_and_map_packages::convert_and_map_packages;
use crate::services::find_package_files::find_package_files;
use crate::utils::parse_configs::{parse_csproj, parse_packages_config};

mod cli;
mod models;
mod services;
mod utils;

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
        println!("{}", "No NuGet packages found in the specified folder".bold().bright_red().to_string());
        return;
    }

    let package_data = convert_and_map_packages(packages);

    for info in package_data {
        if info.is_parsed_ok {
            println!(
                "- {}, version {} {} {}",
                info.name.bold().bright_blue(),
                info.version.italic().bright_blue(),
                if info.is_outdated {
                    "[outdated]".bright_red()
                } else {
                    "".normal()
                },
                if info.is_outdated {
                    format!(" latest: {}", info.latest_version.bright_green())
                } else {
                    "".to_string()
                }
            );

            println!("  license: {}", info.license_expression.bright_yellow());
            println!("  license URL: {}", info.license_url.bright_purple());
            println!("  description: {}", info.description.bright_cyan());
            println!("  project URL: {}", info.project_url.bright_magenta());
            println!("  released at: {}", info.published_date.bright_cyan());
            println!();
        } else {
            println!(
                "- {}, version {}",
                info.name.bold().bright_blue(),
                info.version.italic().bright_blue()
            );
            println!(
                "  {}",
                "Unable to fetch package details".italic().red().to_string()
            );
            println!();
        }
    }
}