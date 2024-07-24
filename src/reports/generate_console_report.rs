use crate::models::project_info::ProjectInfo;
use colored::Colorize;

pub fn generate_console_report(package_data: &Vec<ProjectInfo>) {
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
                // println!("   project URL: {}", nuget.project_url.bright_magenta());
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
