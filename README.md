# Minecraft World Packager
This tool is used to automatically prepare a Minecraft World for release. This project was inspired by:
- https://github.com/shurik204/map-prepare

## Supported features
- Filter files that you want to keep using glob patterns
- Delete chunks that are considered empty (filled only with air / with no entities / with no poi)
- Zip all datapacks and update the level.dat accordingly
- Zip and add a resourcepack to the world if provided
- Set the world name

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
