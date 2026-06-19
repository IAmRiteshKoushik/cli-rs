use std::error::Error;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    println!("Hello, world");
    Ok(())
}

fn get_args() -> MyResult<Config> {}

fn main() {
    println!("Hello, world!");
}
