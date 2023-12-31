use clap::{Parser, ValueEnum};
use regex::Regex;
use std::{error::Error, fs::DirEntry};
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Eq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum EntryTypeMode {
    /// File
    F,
    /// Directory
    D,
    /// Link
    L,
}

#[derive(Parser)]
#[command(author("Yacob (Kobi) Cohen-Arazi <kobi.cohenarazi@gmail.com>"), version("0.1.0"), about("Rust find app"), long_about = None)]
struct Cli {
    #[arg(value_name = "Search paths", default_value("."))]
    paths: Vec<String>,
    #[arg(value_name = "Name", short('n'), long("name"), num_args(0..), help("Name"))]
    names: Vec<Regex>,
    #[arg(
        value_name = "Entry type",
        value_enum,
        short('t'),
        long("type"),
        num_args(0..),
        help("Entry type [possible values: f, d, l]")
    )]
    entry_types: Vec<EntryTypeMode>,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let cli = Cli::parse();
    let et = cli
        .entry_types
        .iter()
        .map(|v| match v {
            EntryTypeMode::F => EntryType::File,
            EntryTypeMode::D => EntryType::Dir,
            EntryTypeMode::L => EntryType::Link,
        })
        .collect();
    let cfg = Config {
        paths: cli.paths,
        names: cli.names,
        entry_types: et,
    };
    Ok(cfg)
}

// fn file_type_included(config: &Config, entry: &walkdir::DirEntry) -> bool {
//     config.entry_types.is_empty()
//         || config.entry_types.iter().any(|et| match et {
//             Link => entry.file_type().is_symlink(),
//             Dir => entry.file_type().is_dir(),
//             File => entry.file_type().is_file(),
//         })
// }

// fn name_included(config: &Config, entry: &walkdir::DirEntry) -> bool {
//     config.names.is_empty()
//         || config
//             .names
//             .iter()
//             .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
// }

// fn handle_entry(config: &Config, entry: &walkdir::DirEntry) {
//     if file_type_included(&config, &entry) && name_included(&config, &entry) {
//         println!("{}", entry.path().display());
//     }
// }

pub fn run(config: Config) -> MyResult<()> {
    for path in &config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    if (config.entry_types.is_empty()
                        || config
                            .entry_types
                            .iter()
                            .any(|entry_type| match entry_type {
                                EntryType::Link => entry.file_type().is_symlink(),
                                EntryType::Dir => entry.file_type().is_dir(),
                                EntryType::File => entry.file_type().is_file(),
                            }))
                        && (config.names.is_empty()
                            || config
                                .names
                                .iter()
                                .any(|re| re.is_match(&entry.file_name().to_string_lossy())))
                    {
                        println!("{}", entry.path().display());
                    }
                }
            }
        }
    }
    Ok(())
}
