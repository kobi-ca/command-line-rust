use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[command(author("Yacob (Kobi) Cohen-Arazi <kobi.cohenarazi@gmail.com>"), version("0.1.0"), about("Rust uniq app"), long_about = None)]
struct Cli {
    #[arg(value_name = "Input file", default_value("-"))]
    in_file: String,
    #[arg(value_name = "Output file")]
    out_file: Option<String>,
    #[arg(short('c'), long("count"), help("Show couns"))]
    count: bool,
}

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let cli = Cli::parse();
    Ok(Config {
        in_file: cli.in_file,
        out_file: cli.out_file,
        count: cli.count,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
