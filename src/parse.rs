
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "css.pest"]
pub struct CssParser;
