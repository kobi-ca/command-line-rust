use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[command(author("Yacob (Kobi) Cohen-Arazi <kobi.cohenarazi@gmail.com>"), version("0.1.0"), about("Rust uniq app"), long_about = None)]
struct Cli {
    #[arg(value_name = "Input file", default_value("-"))]
    in_file: String,
    #[arg(value_name = "Output file")]
    out_file: Option<String>,
    #[arg(short('c'), long("count"), help("Show counts"))]
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
    let cfg = Config {
        in_file: cli.in_file,
        out_file: cli.out_file,
        count: cli.count,
    };
    //println!("{:?}", cfg);
    Ok(cfg)
}

fn print_out(line: &str, out: &Option<String>) {
    print!("{}", line);
}

fn print_out_with_count(line: &str, count: u64, out: &Option<String>) {
    print!("{:>4} {}", count, line);
}

fn print_with_count(count: u64, previous_line: &str, config: &Config) {
    if count > 0 {
        if config.count {
            print_out_with_count(previous_line, count, &config.out_file);
        } else {
            print_out(previous_line, &config.out_file)
        }
    }
}

// anoter way is to create a closure and call it. Book recommending it.
// let print = |count: u64, text: &str| {
//    if count > 0 {... } else { ... }
// }
//
// However, I prefer to have it as a separate function.

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut current_line = String::new();
    let mut previous_line = String::new();
    let mut count: u64 = 0;
    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };
    loop {
        let bytes = file.read_line(&mut current_line)?;
        if bytes == 0 {
            break;
        }
        if current_line.trim_end() != previous_line.trim_end() {
            print_with_count(count, &previous_line, &config);
            previous_line = current_line.clone();
            count = 0;
        }
        count += 1;
        current_line.clear();
    }
    if count > 0 {
        print_with_count(count, &previous_line, &config);
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
