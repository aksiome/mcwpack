# Minecraft World Packager

[![Source Code](https://img.shields.io/badge/source-aksiome/mcwpack-4078C0.svg?style=flat-square&labelColor=555555&logo=github)](https://github.com/aksiome/mcwpack)
[![Software License](https://img.shields.io/github/license/aksiome/mcwpack?style=flat-square)](https://github.com/aksiome/mcwpack/blob/master/LICENSE)

This tool is used to automatically prepare a Minecraft World for release. This project was inspired by:
- https://github.com/shurik204/map-prepare

#### Also exists as a github action
[![`minecraft-package`](https://img.shields.io/badge/aksiome/minecraft--package-6f42c1?style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/aksiome/minecraft-package)

## Features
- Filter files that you want to keep using glob patterns
- Delete chunks that are considered empty (filled only with air / with no entities / with no poi)
- Zip all datapacks and update the level.dat accordingly
- Zip and add a resourcepack to the world if provided
- Zip additional files (Readme, ...)
- Filter scores and objectives
- Set the level.dat world name


## How to use

### Download
[![Windows](https://img.shields.io/badge/windows-0068B6?style=for-the-badge&logo=windows)](https://github.com/aksiome/mcwpack/releases/latest/download/mcwpack-windows.zip)
[![Linux](https://img.shields.io/badge/linux-D97120?style=for-the-badge&logo=linux)](https://github.com/aksiome/mcwpack/releases/latest/download/mcwpack-linux.tar.gz)
[![macOS](https://img.shields.io/badge/macos-777777?style=for-the-badge&logo=apple)](https://github.com/aksiome/mcwpack/releases/latest/download/mcwpack-macos.zip)

You can either run the program and follow the instructions or use it as a command:
```
Usage: mcwpack [WORLD_PATH] [OPTIONS]

Arguments:
  [WORLD_PATH]

Options:
  -z <ZIP_PATH>         Set the output zip
  -d <DIR_PATH>         Set the output directory
  -c <CONFIG_FILE>      Use the given config file
  -v                    Show debug trace
  -q                    Silence warning
  -h, --help            Print help
  -V, --version         Print version
```

## Compiling from source

You need to have **[cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)** installed on your system.

Download the archive or clone the repository:
```
git clone https://github.com/aksiome/mcwpack.git
```

Run the following command inside the repository:
```
cargo build --release
```
The `mcwpack` binary should be created inside the `target/release` directory.

## Contributions

Anyone can contribute to this repository. Please do so by posting issues when you've found something that is unexpected or sending a pull request for improvements.
