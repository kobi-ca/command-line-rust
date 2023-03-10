use clap::{Arg, ArgAction, Command};
use std::fs::File;
use std::{
    error::Error,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;
const COUNT_DEFAULT_VALUE: &str = "10";

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
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Yacob (Kobi) Cohen-Arazi <kobi.cohenarazi@gmail.com")
        .about("Rust head app")
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(usize))
                .conflicts_with("lines")
                .help(
                    "print the first K bytes of each file\
                with the leading '-', print all but the last\
                K bytes of each file",
                ),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .action(ArgAction::Set)
                .default_value(COUNT_DEFAULT_VALUE) // FIXME cannot pass string. must be 'static
                .value_parser(clap::value_parser!(usize))
                .help(
                    "print the first K lines instead of the \
                first 10; with the leading '-', print all but the \
                last K lines of each file",
                ),
        )
        .arg(
            Arg::new("files")
                .action(ArgAction::Append)
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-"),
        )
        .get_matches();

    let bytes: Option<usize> = matches.get_one("bytes").copied();
    if let Some(bytes) = bytes {
        if bytes == 0 {
            return Err(From::from("Illegal byte count -- 0"));
        }
    }

    let lines = *matches.get_one("lines").expect("cannot get lines");
    if lines == 0 {
        return Err(From::from("Illegal line count -- 0"));
    }

    let files: Vec<String> = matches.get_many("files").unwrap().cloned().collect();
    println!("Config: {:?} {} {:#?}", bytes, lines, files);
    let final_config = Config::new(files, lines, bytes);
    println!("Config debug: {:#?}", final_config);
    Ok(final_config)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}\n", filename, err),
            Ok(buf_read) => {
                if config.files.len() > 1 {
                    println!("==> {} <==", filename);
                }
                //print_file(&config, buf_read)?;
            }
        }
    }
    Ok(())
}

// fn print_file(config: &Config, buf_read: Box::<dyn BufRead>) -> MyResult<()> {
//     if config.bytes.is_some() {
//         print_file_by_chars(config, buf_read)?
//     } else {
//         print_file_by_lines(config, buf_read)?
//     }
//     Ok(())
// }

// fn print_file_by_chars(config: &Config, buf_read: Box::<dyn BufRead>) -> MyResult<()> {
//     for line in buf_read.lines() {
//         match line {
//             Ok(charaters) => print_n_chars(config.bytes.unwrap(), &charaters)?,
//             Err(_) => break,
//         }
//     }
//     Ok(())
// }

// fn print_n_chars(bytes: usize, charaters: &str) -> MyResult<()> {
//     for (count, c) in charaters.chars().enumerate() {
//         print!("{}", c);
//         // enumerate starts with 0
//         if count + 1 == bytes {
//             break;
//         }
//     }
//     Ok(())
// }

// fn print_file_by_lines(config: &Config, buf_read: Box::<dyn BufRead>) -> MyResult<()> {
//     for (count, line) in buf_read.lines().enumerate() {
//         match line {
//             Ok(l) => println!("{}", l),
//             Err(_) => break,
//         }
//         // enumerate starts with 0
//         if count + 1  == config.lines {
//             break;
//         }
//     }

//     Ok(())
// }
