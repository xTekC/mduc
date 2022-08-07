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
    let /*mut*/ usage = String::from("Usage: markdup <usage.md>");
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
    
    match args.len() {
        2 => parse_md_file(&args[1]),
        _ => {
            println!("[ ERROR ] Invalid Input\n");
            info();
        }
    }


}
