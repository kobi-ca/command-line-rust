use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    // could use pathbuf as in https://docs.rs/clap/latest/clap/_tutorial/index.html
    // with value_parser!(PathBuf)
    files: Vec<String>,
    // FIXME Option<> ?
    number_lines: bool,
    number_nonblank_lines: bool,
}

impl Config {
    pub fn new(files: Vec<String>, number_lines: bool,
            number_nonblank_lines: bool) -> Config {
        Config {
            files,
            number_lines,
            number_nonblank_lines,
        }
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .author("Kobi Cohen-Arazi <kobi.cohenarazi@gmail.com>")
        .version("0.1.0")
        .about("Cat files or stdin")
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number lines")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .action(clap::ArgAction::SetTrue)
                .short('b')
                .long("number-nonblank")
                .help("Number nonblank lines"),
        )
        .arg(
            Arg::new("files")
                .value_parser(clap::value_parser!(String))
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .action(clap::ArgAction::Append),
        )
        .get_matches();
    let number = matches.get_flag("number");
    let number_nonblank = matches.get_flag("number_nonblank");
    let files = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|v| v.to_owned())
        .collect();
    Ok(Config::new(files, number, number_nonblank))
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read_file(fileio: Box<dyn BufRead>, config: &Config) -> MyResult<()>{
    let mut idx = 1;
    let need_to_print_idx = config.number_nonblank_lines || config.number_lines;
    for line in fileio.lines() {
        let l = line?;
        if !need_to_print_idx {
            println!("{}", l);
            continue;
        }
        if l.is_empty() && config.number_nonblank_lines {
            println!();
            continue;
        }
        if !l.is_empty() && config.number_lines {
            println!("{:>6}\t{}", idx, l);
            idx += 1;
            continue;
        }
        println!("{:>6}\t{}", idx, l);
        idx += 1;
    }
    Ok(())
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(fileio) => {
                read_file(fileio, &config)?;
            },
        }
    }
    Ok(())
}
