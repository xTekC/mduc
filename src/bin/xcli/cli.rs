/******************************
 *  Copyright (c) xTekC.      *
 *  Licensed under MPL-2.0.   *
 *  See LICENSE for details.  *
 *                            *
 ******************************/

use clap::Parser;
use mduc::xcore::core::parse;

/// Compile Markdown to HTML
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
        false => parse(&cli.mdfile).await,
    }
}
