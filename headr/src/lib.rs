use clap::{App, Arg};
use std::{error::Error, io::{self, BufRead, BufReader}, f32::consts::E};
use std::fs::File;

type MyResult<T> = Result<T, Box<dyn Error>>;
const DEFAULT_VALUE: usize = 10;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

impl Config {
    pub fn new(files: Vec<String>, lines: usize, bytes: Option<usize>) -> Self {
        Config {
            files,
            lines,
            bytes,
        }
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Yacob (Kobi) Cohen-Arazi <kobi.cohenarazi@gmail.com")
        .about("Rust head")
        .arg(
            Arg::with_name("bytes")
                .short('c')
                .long("bytes")
                .takes_value(true)
                .conflicts_with("lines")
                .help(
                    "print the first K bytes of each file\
                with the leading '-', print all but the last\
                K bytes of each file",
                ),
        )
        .arg(
            Arg::with_name("lines")
                .short('n')
                .long("lines")
                .default_value(format!("{}", DEFAULT_VALUE).as_str())
                .takes_value(true)
                .help(
                    "print the first K lines instead of the \
                first 10; with the leading '-', print all but the \
                last K lines of each file",
                ),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .allow_invalid_utf8(true)
                .multiple_values(true)
                .default_value("-"),
        )
        .get_matches();

    let bytes = matches.value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("Illegal byte count -- {}", e))?;

    let lines = matches.value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("Illegal line count -- {}", e))?;

    let files = matches.values_of_lossy("files").unwrap();
    Ok(Config::new(
        files,
        lines.unwrap(),
        bytes,
    ))
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    Ok(())
}

pub fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(num)if num > 0 => Ok(num),
        _ => Err(From::from(val)),
    }
}

fn open(filename: &str) ->MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _   => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
