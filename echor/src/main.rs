use clap::{App,Arg};


fn main() {
    //println!("Hello, world!");
    //println!("{:?}", std::env::args());
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Kobi")
        .about("echor prog")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("input text")
                .required(true)
                .min_values(1)
                .allow_invalid_utf8(true)
        )
        .arg(
            Arg::with_name("omit_newline")
            .short('n')
            .help("Do not print newline")
            .takes_value(false)
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    // let mut ending = "\n";
    // if omit_newline {
    //     ending = "";
    // }
    let ending = if omit_newline {""} else {"\n"};

    print!("{}{}", text.join(" "), ending);
    // println!("{:#?}", matches);
}

