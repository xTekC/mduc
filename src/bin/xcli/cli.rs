use clap::Parser;
use mduc::xcore::parse::parse_md_file;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Markdown file to compile
    mdfile: String,
}

pub async fn cli_main() {
    let cli = Cli::parse();

    match cli.mdfile.is_empty() {
        true => (),
        false => parse_md_file(&cli.mdfile).await,
    }
}
