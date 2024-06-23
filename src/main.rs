use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;
use clap::Parser;
use colored::Colorize;

use crate::cli::Cli;
use crate::models::project_info::ProjectInfo;
use crate::services::process_projects_data::process_projects_data;
use crate::services::find_package_files::find_package_files;

mod cli;
mod models;
mod services;
mod utils;

fn main() {
    // print!("{esc}c", esc = 27 as char);

    let start_time = Instant::now();

    let args = Cli::parse();

    let folder = args.folder.to_string_lossy().to_string();
    let package_files = find_package_files(&folder, args.max_depth);

     let project_dirs: Vec<PathBuf> = package_files.iter()
        .filter(|file| file.ends_with(".csproj"))
        .map(|file| PathBuf::from(file).parent().unwrap().to_path_buf())
        .collect();

    let project_infos = process_projects_data(project_dirs);

    if project_infos.is_empty() {
        println!("{}", "No NuGet packages found in the specified folder".bold().bright_red().to_string());
        return;
    }

    // for the future
    // project_infos.sort_by(|a, b| a.project_name.cmp(&b.project_name));
    // project_infos.iter_mut().for_each(|project| {
    //     project.packages.sort_by(|a, b| a.name.cmp(&b.name));
    // });

    print_packages_info(&project_infos);

    if args.report {
        generate_markdown_report(project_infos, folder.clone());
    }

    println!("\nCompleted in: {:.2?} seconds", start_time.elapsed().as_secs_f64());
}

fn print_packages_info(package_data: &Vec<ProjectInfo>) {
    for project in package_data {
        println!(
            "{}",
            format!("Project: {}", project.project_name.bold().bright_green())
        );
        
        if project.packages.is_empty() {
            println!(" - No packages found in this project\n");
            continue;
        }

        for nuget in project.packages.iter() {
            if nuget.is_parsed_ok {
                println!(
                    " - {}, version {} {} {}",
                    nuget.name.bold().bright_blue(),
                    nuget.version.italic().bright_blue(),
                    if nuget.is_outdated {
                        "[outdated]".bright_red()
                    } else {
                        "".normal()
                    },
                    if nuget.is_outdated {
                        format!(" latest: {}", nuget.latest_version.bright_green())
                    } else {
                        "".to_string()
                    }
                );

                println!("   license: {}", nuget.license_expression.bright_yellow());
                println!("   license URL: {}", nuget.license_url.bright_purple());
                println!("   description: {}", nuget.description.bright_cyan());
                println!("   project URL: {}", nuget.project_url.bright_magenta());
                println!("   released at: {}", nuget.published_date.bright_cyan());
                println!();
            } else {
                println!(
                    " - {}, version {}",
                    nuget.name.bold().bright_blue(),
                    nuget.version.italic().bright_blue()
                );
                println!(
                    "   {}",
                    "Unable to fetch package details".italic().red().to_string()
                );
                println!();
            }
        }
    }
}

fn generate_markdown_report(package_data: Vec<ProjectInfo>, save_folder_path: String) {
    let mut report = String::new();

    report.push_str("# NuGet Packages Report\n\n");

    for project in package_data {
        report.push_str(&format!("### Project {}\n", project.project_name));

        if project.packages.is_empty() {
            report.push_str("No packages found in this project\n");
            continue;
        }
        
        report.push_str("| Package Name | Version | Latest? | License | Description |\n");
        report.push_str("| --- | --- | --- | --- | --- |\n");

        for package in project.packages.iter() {
            if package.is_parsed_ok {
                let outdated = if package.is_outdated {
                    format!("🔸 {}", package.latest_version)
                } else {
                    "✅".to_string()
                };

                report.push_str(&format!(
                    "| **[{}]({})** | {} | {} | [{}]({}) | {} |\n",
                    package.name,
                    package.project_url,
                    package.version,
                    outdated,
                    package.license_expression,
                    package.license_url,
                    package.description
                ));
            } else {
                report.push_str(&format!(
                    "| {} | {} | - | - | {} | \n",
                    package.name,
                    package.version,
                    "🛑 error fetching details"
                ));
            }
        }
    }

    report.push_str("\n---\n");
    report.push_str("_Generated by [nuget-scanner](https://github.com/RustamIrzaev/nuget_scanner)_");

    let report_path = format!("{}/LICENSE_REPORT.md", save_folder_path.clone());
    let mut file = File::create(&report_path).expect("Unable to create file");
    file.write_all(report.as_bytes()).expect("Unable to write data");

    println!("\nReport generated: {}", report_path.bright_blue());
}