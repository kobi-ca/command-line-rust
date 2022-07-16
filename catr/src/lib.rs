use std::error::Error;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    // FIXME Option<> ?
    number_lines: bool,
    number_nonblank_lines: bool,
}

impl Config {
    pub fn new(files: &Vec<String>,
        number_lines: bool,
        number_nonblank_lines: bool) -> Config {
        Config{files: files.clone(),
            number_lines,
            number_nonblank_lines}
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Kobi Cohen-Arazi <kobi.cohenarazi@gmail.com>")
        .arg(
            Arg::with_name("number")
                .short('n')
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank")
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short('b')
                .takes_value(false)
                .help("Number nonblank lines")
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s) [default: -]")
                .default_value("-")
                .allow_invalid_utf8(true)
                .multiple_values(true)
                //.required(true)
                //.min_values(1)
        )
        .get_matches();
    let number = matches.is_present("number");
    let number_nonblank = matches.is_present("number_nonblank");
    let files = matches.values_of_lossy("files").unwrap();
    Ok(Config::new(&files,
        number,
        number_nonblank))
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}
