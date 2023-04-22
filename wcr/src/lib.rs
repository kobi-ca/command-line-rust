use clap::{Arg, ArgAction, Command};
use std::{
    error::Error,
    io::{BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
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

    let mut l = matches.get_flag("lines");
    let mut w = matches.get_flag("words");
    let mut b = matches.get_flag("bytes");
    let c = matches.get_flag("chars");
    if [l, w, b, c].iter().all(|v| !v) {
        l = true;
        w = true;
        b = true;
    }
    Ok(Config {
        files: matches.get_many("files").unwrap().cloned().collect(),
        lines: l,
        words: w,
        bytes: b,
        chars: c,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(std::fs::File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    println!("{:#?}", config);
    Ok(())
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
    }
}
