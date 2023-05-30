use crate::EntryType::*;
use clap::{Parser, ValueEnum};
use regex::Regex;
use std::error::Error;

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
    f,
    /// Directory
    d,
    /// Link
    l,
}

#[derive(Parser)]
#[command(author("Yacob (Kobi) Cohen-Arazi <kobi.cohenarazi@gmail.com>"), version("0.1.0"), about("Rust find app"), long_about = None)]
struct Cli {
    #[arg(value_name = "Search paths", default_value("."))]
    paths: Vec<String>,
    #[arg(value_name = "Name", short('n'), long("name"), help("Name"))]
    names: Vec<Regex>,
    #[arg(
        value_name = "Entry type",
        value_enum,
        short('t'),
        long("type"),
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
            EntryTypeMode::f => EntryType::File,
            EntryTypeMode::d => EntryType::Dir,
            EntryTypeMode::l => EntryType::Link,
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
    println!("{:?}", config);
    Ok(())
}
