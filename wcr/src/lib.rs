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
        .get_matches();

    Ok(Config {
        files: vec![],
        lines: false,
        words: false,
        bytes: false,
        chars: false,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
