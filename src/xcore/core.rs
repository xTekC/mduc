/******************************
 *  Copyright (c) xTekC.      *
 *  Licensed under MPL-2.0.   *
 *  See LICENSE for details.  *
 *                            *
 ******************************/

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub async fn parse(the_file: &str) {
    println!("\n[ INFO ] Attempting to parse {the_file}...");

    let tokens = process_file(the_file);

    create_html_file(the_file, &tokens);

    println!("[ INFO ] Parsing Complete!");
    println!("[ INFO ] Created .html file!")
}

fn process_file(the_file: &str) -> Vec<String> {
    let input_file = Path::new(the_file);
    let file = File::open(input_file).expect("[ ERROR ] Failed to open file!");
    let reader = BufReader::new(file);

    let mut htag: bool = false;
    let mut ptag: bool = false;
    let mut tokens: Vec<String> = Vec::new();

    for line in reader.lines() {
        let (line_tokens, is_htag, is_ptag) = process_line(&line.unwrap(), htag, ptag);
        htag = is_htag;
        ptag = is_ptag;
        tokens.push(line_tokens);
    }

    tokens
}

fn process_line(line_contents: &str, mut htag: bool, mut ptag: bool) -> (String, bool, bool) {
    let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
    let mut output_line = String::new();

    match first_char.pop() {
        Some('#') => {
            if ptag {
                ptag = false;
                output_line.push_str("</p>\n");
            }

            if htag {
                output_line.push_str("</h1>\n");
            }

            htag = true;
            output_line.push_str("\n\n<h1>");
            output_line.push_str(&line_contents[2..]);
        }
        _ => {
            if !ptag {
                ptag = true;
                output_line.push_str("<p>");
            }
            output_line.push_str(line_contents);
        }
    }

    if ptag {
        ptag = false;
        output_line.push_str("</p>\n");
    }

    if htag {
        htag = false;
        output_line.push_str("</h1>\n");
    }

    (output_line, htag, ptag)
}

fn create_html_file(the_file: &str, tokens: &[String]) {
    let mut output_file = String::from(&the_file[..the_file.len() - 3]);
    output_file.push_str(".html");
    let mut outfile = File::create(output_file).expect("[ ERROR ] Could not create output file!");

    for line in tokens {
        outfile
            .write_all(line.as_bytes())
            .expect("[ ERROR ] Could not write to output file!");
    }
}
