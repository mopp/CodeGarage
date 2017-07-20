// https://employment.en-japan.com/engineerhub/entry/2017/07/19/110000
extern crate regex;
use regex::Regex;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;

fn main() {
    match grep() {
        Err(msg) => println!("{}", msg),
        Ok(_)    => (),
    }
}

fn grep() -> Result<(), String> {
    static USAGE_MESSAGE: &'static str = "rsgrep PATTERN FILENAM";

    let pattern  = env::args().nth(1).ok_or(USAGE_MESSAGE.to_string())?;
    let filename = env::args().nth(2).ok_or(USAGE_MESSAGE.to_string())?;
    let reg      = Regex::new(&pattern).map_err(|e| e.to_string())?;
    let input    =
        File::open(&filename)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;

    for line in input.lines() {
        let line = line.map_err(|e| e.to_string())?;

        if reg.is_match(&line) {
            println!("{}", line);
        }
    }

    Ok(())
}
