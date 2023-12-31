use clap::{Parser, ValueEnum};
use regex::Regex;
use std::error::Error;
use walkdir::{DirEntry, WalkDir};

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

pub fn run(config: Config) -> MyResult<()> {
    let type_filter = |entry: &DirEntry| {
        config.entry_types.is_empty()
            || config
                .entry_types
                .iter()
                .any(|entry_type| match entry_type {
                    EntryType::Link => entry.path_is_symlink(),
                    EntryType::Dir => entry.file_type().is_dir(),
                    EntryType::File => entry.file_type().is_file(),
                })
    };

    let name_filter = |entry: &DirEntry| {
        config.names.is_empty()
            || config
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in &config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();
        println!("{}", entries.join("\n"));
    }
    Ok(())
}
