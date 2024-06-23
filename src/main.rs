use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;
use std::time::Instant;

use crate::{
    cli::Cli,
    reports:: {
        generate_console_report::generate_console_report,
        generate_markdown_report::generate_markdown_report,
    },
    services:: {
        find_package_files::find_package_files,
        process_projects_data::process_projects_data,
    },
};

mod cli;
mod models;
mod reports;
mod services;
mod utils;

fn main() {
    // print!("{esc}c", esc = 27 as char);

    let start_time = Instant::now();

    let args = Cli::parse();

    let folder = args.folder.to_string_lossy().to_string();
    let package_files = find_package_files(&folder, args.max_depth);

    let project_dirs: Vec<PathBuf> = package_files
        .iter()
        .filter(|file| file.ends_with(".csproj"))
        .map(|file| PathBuf::from(file).parent().unwrap().to_path_buf())
        .collect();

    let project_infos = process_projects_data(project_dirs);

    if project_infos.is_empty() {
        println!(
            "{}",
            "No NuGet packages found in the specified folder"
                .bold()
                .bright_red()
                .to_string()
        );
        return;
    }

    // for the future
    // project_infos.sort_by(|a, b| a.project_name.cmp(&b.project_name));
    // project_infos.iter_mut().for_each(|project| {
    //     project.packages.sort_by(|a, b| a.name.cmp(&b.name));
    // });

    generate_console_report(&project_infos);

    if args.report {
        generate_markdown_report(project_infos, folder.clone());
    }

    println!(
        "\nCompleted in: {:.2?} seconds",
        start_time.elapsed().as_secs_f64()
    );
}