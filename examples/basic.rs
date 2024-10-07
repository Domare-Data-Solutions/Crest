use clap::{command, Parser as Cli};
use pest_consume::Parser as _;

use std::ffi::OsString;
use std::fs::read_to_string;

use peacock_crest::parse::{CssParser, Rule};

#[derive(Cli)]
#[command(
    name = "validate-grammar",
    about = "Run the generated css parser on the provided source"
)]
struct ValidationArgs {
    // /// the name of the parser rule to evaluate
    // rule: String,

    /// parse contents of file at this path
    source_path: String,
}

fn main() -> Result<(), pest_consume::Error<Rule>> {
    let arguments = ValidationArgs::parse();
    let path = OsString::from(arguments.source_path);
    let source = read_to_string(path).unwrap();

    let parsed = CssParser::parse(Rule::Css, source.as_str())?;
    let sheet_node = parsed.single()?;
    let stylesheet = CssParser::Css(sheet_node)?;

    for rule in stylesheet.iter() {
        println!("{rule:?}");
    }

    Ok(())
}
