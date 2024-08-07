# NuGet Scanner

## Overview

The `NuGet Scanner` is a powerful tool designed for C# (.NET) engineers to easily scan their projects, identify used NuGet packages & licenses, check if those packages are outdated, and generate a detailed report.
This application leverages Rust's performance and safety to provide quick and reliable results.

> This is the successor of the [NuGet Helper](https://github.com/RustamIrzaev/NuGetHelper) that was released many years ago.

<p align="center">
  <img width="70%" alt="main_menu" src="https://github.com/RustamIrzaev/nuget_scanner/assets/352703/adfc04c4-1e74-4c60-a8be-4a8f48e260c4">
</p>

## Features

- **Scan C# Projects**. Automatically detects and processes `.csproj` and `packages.config` files within the specified directory.
- **Identify NuGet Packages**. Extracts the list of NuGet packages used in each project.
- **Fetch Package Details**. Retrieves comprehensive information about each NuGet package from the NuGet API, including:
  - Package Name _(the name of the NuGet package)_
  - Current Version _(the version of the package currently used in the project)_
  - Latest Version _(the latest available version of the package)_
  - License URL _(a URL pointing to the license information of the package)_
  - License Type _(the license type or expression associated with the package)_
  - Published Date _(the date when the current version of the package was published)_
- **Check for Outdated Packages**. Compares the current version of each package with the latest available version to identify outdated packages.
- **Generate Detailed Reports**. Outputs a detailed report in markdown file for each project, listing all NuGet packages along with their details and version status.
- **Multiple Project Support**. Capable of scanning directories containing multiple C# projects, ensuring each project is analyzed individually.
- **Execution Time Measurement**. Measures and displays the total execution time of the scanning process.

### Notes

- The application uses the NuGet API to fetch package details. Therefore, an active internet connection is required.
- The inner parser uses NuGet SemVer2 so not every (old) package may be parsed correctly.

## Usage

### Prerequisites

- **Rust**. Ensure that Rust is installed on your system. You can install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Building the Project

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/RustamIrzaev/nuget_scanner.git
cd nuget_scanner
cargo build --release    # builds the project

cd target/release        # switch to where the executable is placed
./nuget-scanner -f <FOLDER> -r # run nuget-scanner! (see options below)
```

### Running the Application

To run the application, specify the directory to scan and optionally the maximum depth for scanning:

```bash
./nuget-scanner -f /path/to/your/csharp/project -r
```

### Parameters

- `-f` or `--folder`: path to the directory containing the C# project(s).
- `-r` or `--report`: to generate a report
- `--max-depth`: (_Optional_) specifies the maximum depth for scanning directories. Default is 10.

## Example output

```markdown
Project: MyProject

- NuGet Package #1, version 13.0.3
  license: MIT
  license URL: https://github.com/RustamIrzaev/nuget_scanner/license.md
  description: Package description
  project URL: https://github.com/RustamIrzaev/nuget_scanner
  released at: 08 Mar 2023
- NuGet Package #2, version 12.0.3
  license: MIT
  license URL: https://github.com/RustamIrzaev/nuget_scanner/license.md
  description: Package description
  project URL: https://github.com/RustamIrzaev/nuget_scanner
  released at: 08 Mar 2024

Completed in: 0.23 seconds
```

### Example report

Demo report can be found [here](DEMO_LICENSE_REPORT.md).

<p align="center">
  <img width="70%" alt="main_menu" src="https://github.com/RustamIrzaev/nuget_scanner/assets/352703/afc993d6-ad40-47f3-a0e1-8249eea7bb52">
</p>

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE.MD) file for details.
