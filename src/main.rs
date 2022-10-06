use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("). ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

// called when a md file is passed in terminal
fn parse_md_file(_the_file: &str) {
    short_banner();
    println!("\n[ INFO ] Attempting to parse {}...", _the_file);
    // create a path var from the_file
    let input_file = Path::new(_the_file);

    // attempt to open the file
    let file = File::open(&input_file).expect("[ ERROR ] Failed to open file!");

    // heading tag
    let mut htag: bool = false;

    // paragraph tag
    let mut ptag: bool = false;

    // [token]: keyword, operator, seperator or string literal
    // holds all of output_lines iterations from for loop to output file
    let mut tokens: Vec<String> = Vec::new();

    // read the file
    let reader = BufReader::new(file);

    // loop through the lines
    for line in reader.lines() {
        // for each line, unwrap it
        let line_contents = line.unwrap();

        // .chars(): convert line_contents to sequence of chars
        // .take(1): take first elem of the iterable object
        // .collect(): convert everything into a collection (Vec)
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();

        // new string to hold valid HTML
        // will be pushed to tokens var after processing
        let mut output_line = String::new();

        match first_char.pop() {
            // first char is #
            Some('#') => {
                // check if paragraph tag is set
                if ptag {
                    // if set, unset it
                    ptag = false;
                    // send a closing </p> tag and newline
                    // to output_line string
                    output_line.push_str("</p>\n");
                }

                #[allow(unused)]
                // check if heading tag is set
                if htag {
                    // if set, unset it
                    htag = false;
                    // send a closing </h1> tag and newline
                    // to output_line string
                    output_line.push_str("</h1>\n")
                }

                // set heading tag to true
                htag = true;
                // push <h1> tag to output_line
                output_line.push_str("\n\n<h1>");
                // get all line_contents except the first 2 chars
                // push to output_line
                output_line.push_str(&line_contents[2..]);
            }

            // first char is not #
            _ => {
                // check if paragraph tag is false
                if !ptag {
                    // set it to true
                    ptag = true;
                    // push a <p> to the output_line
                    output_line.push_str("<p>");
                }

                // push the whole line to the output_line
                output_line.push_str(&line_contents);
            }
        }; // end of match

        // if paragraph tag open,
        if ptag {
            // close it
            ptag = false;
            // push </p>
            output_line.push_str("</p>\n");
        }

        // if heading tag open
        if htag {
            // close it
            htag = false;
            // push </h1>
            output_line.push_str("</h1>\n");
        }

        // to avoid pushing blank lines
        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    } // end of "for line in reader.lines()" block

    // loop over tokens and print
    // for t in &tokens {
    //     println!("{}", t);
    // }

    // get all but the last 3 chars of _the_file (the file type .md)
    let mut output_file = String::from(&_the_file[.._the_file.len() - 3]);

    // push .html to the end of the file name
    output_file.push_str(".html");

    // create output file
    let mut outfile = File::create(output_file).expect("[ ERROR ] Could not create output file!");

    // write each element to the output file
    for line in &tokens {
        // for each line in tokens,
        // write each line as a byte sequence to the outfile
        outfile
            .write_all(line.as_bytes())
            .expect("[ ERROR ] Could not write to output file!");
    }

    println!("[ INFO ] Parsing Complete!");
}

// output: title, version and description
fn short_banner() {
    println!("{}. ", title());
}

// output: short_banner(), author, homepage and usage
fn long_banner() {
    short_banner();
    let mut author = String::from("Written by: ");
    author.push_str(env!("CARGO_PKG_AUTHORS"));
    let mut homepage = String::from("Homepage: ");
    homepage.push_str(env!("CARGO_PKG_HOMEPAGE"));
    let /*mut*/ usage = String::from("Usage: mduc <usage.md>");
    println!("{}", author);
    println!("{}", homepage);
    println!("{}", usage);
}

fn info() {
    println!("{:?}", long_banner());
}

fn main() {
    // collect all arguments in a vector
    let args: Vec<String> = std::env::args().collect();

    // checks to make sure there are only 2 elements in args Vec
    // elem 1: name of program
    // elem 2: markdown file
    match args.len() {
        2 => parse_md_file(&args[1]),
        _ => {
            println!("[ ERROR ] Invalid Input\n");
            info();
        }
    }
}
