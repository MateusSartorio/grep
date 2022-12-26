use std::env;
use std::fs;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = lib::parse_config(&args);


}
