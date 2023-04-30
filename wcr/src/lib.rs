use clap::Parser;
use std::{
    error::Error,
    io::{BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[command(author("Yacob (Kobi) Cohen-Arazi <kobi.cohenarazi@gmail.com>"), version("0.1.0"), about("Rust wc app"), long_about = None)]
struct Cli {
    #[arg(value_name = "FILE", default_value("-"))]
    file: Vec<String>,
    #[arg(short('l'), long("lines"), help("Show line count"))]
    lines: bool,
    #[arg(short('w'), long("words"), help("Show word count"))]
    words: bool,
    #[arg(
        short('c'),
        long("bytes"),
        conflicts_with("chars"),
        help("Show byte count")
    )]
    bytes: bool,
    #[arg(short('m'), long("chars"), help("Show character count"))]
    chars: bool,
}

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
    let cli = Cli::parse();
    let mut cfg = Config {
        files: cli.file,
        lines: cli.lines,
        words: cli.words,
        bytes: cli.bytes,
        chars: cli.chars,
    };
    if [cfg.lines, cfg.words, cfg.bytes, cfg.chars]
        .iter()
        .all(|v| !v)
    {
        cfg.lines = true;
        cfg.words = true;
        cfg.bytes = true;
    }
    Ok(cfg)
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
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
