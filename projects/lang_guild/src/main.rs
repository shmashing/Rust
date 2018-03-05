extern crate reqwest;
extern crate url;
extern crate regex;

use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use url::Url;
use regex::Regex;

#[macro_use]
extern crate error_chain;

error_chain!{
    foreign_links {
        Io( std::io::Error );
        Reqwest( reqwest::Error );
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    
    for url in args {
        let resp = get_text( url );
        write_to_file( resp.unwrap() );
    }
}

fn get_text(url_str: String) -> Result<String> {
    let url = Url::parse( &url_str ).unwrap();
    let mut resp = reqwest::get( url )?;
    let text = resp.text()?;
    
    Ok( text )
}

fn write_to_file(text: String) {
    let mut num_file = OpenOptions::new().append(true).open("numbers.txt").unwrap();
    let mut text_file = OpenOptions::new().append(true).open("letters.txt").unwrap();

    let alpha = Regex::new(r"[a-zA-Z]").unwrap();
    let numeric = Regex::new(r"[0-9]").unwrap();

    for (_, c) in text.chars().enumerate() {

        let s = c.to_string();

        if alpha.is_match(&s) {
            text_file.write(s.as_bytes());
        } else if numeric.is_match(&s) {
            num_file.write(s.as_bytes());
        }
    }
}