use clap::{Arg, ArgAction, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .author("Yacob (Kobi) Cohen-Arazi <kobi.cohenarazi@gmail.com>")
        .about("Rust wc app")
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .action(ArgAction::SetTrue)
                .conflicts_with("chars")
                .help("Show byte count"),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .action(ArgAction::SetTrue)
                .help("Show character count"),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .action(ArgAction::SetTrue)
                .help("Show line count"),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .action(ArgAction::SetTrue)
                .help("Show word count"),
        )
        .arg(
            Arg::new("files")
                .action(ArgAction::Append)
                .value_name("FILE")
                .help("Input files(s)")
                .default_value("-"),
        )
        .get_matches();

    Ok(Config {
        files: matches.get_many("files").unwrap().cloned().collect(),
        lines: matches.get_flag("lines"),
        words: matches.get_flag("words"),
        bytes: matches.get_flag("bytes"),
        chars: matches.get_flag("chars"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
