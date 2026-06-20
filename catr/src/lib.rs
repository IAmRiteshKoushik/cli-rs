use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ritesh Koushik")
        .about("Rust implementation of cat")
        .usage("catr [FLAGS] [FILE]...")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .required(false)
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .takes_value(false)
                .help("number all output lines")
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .takes_value(false)
                .help("number nonempty output lines, overrides -n"),
        )
        .get_matches();

    // let file = matches
    //     .value_of("file")
    //     .map(str::to_string)
    //     .unwrap_or(String::from("-"));
    // let number_lines = matches.is_present("number_lines");
    // let number_nonblank_lines = matches.is_present("number_nonblank_lines");

    // if number_lines && number_nonblank_lines {
    //     return Err(Box::from(
    //         "error: The argument '--number-nonblank' cannot be used with '--number'\n",
    //     ));
    // }
    // Ok(Config {
    //     files: vec![file],
    //     number_lines: number_lines,
    //     number_nonblank_lines: number_nonblank_lines,
    // })

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

// BufRead is a trait, the return type is a FatPointer which is going to return
// a type that impelments that Trait. Now, if hyphen is found, then a BufReader
// attaches to std::io as it impelemnents the trait. But, if that is not that
// case, then by default, it opens the file and returns a reader to that file
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            // Ok(_) => println!("Opened {}", filename),
            Ok(buffer) => {
                let mut skipped_line_count = 0;
                for (line_num, line) in buffer.lines().enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:6}\t{line}", line_num + 1);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            println!("{:6}\t{line}", line_num + 1 - skipped_line_count);
                        } else {
                            skipped_line_count += 1;
                            println!()
                        }
                    } else {
                        println!("{line}");
                    }
                }
            }
        }
    }
    Ok(())
}
