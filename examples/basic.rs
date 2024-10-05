use clap::{command, Parser as Cli};
use pest::iterators::FlatPairs;
use pest::Parser as _;

use std::collections::HashMap;
use std::fs::read_to_string;

use peacock_crest::parse::{CssParser, Rule};

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$($v,)*]))
    }};
}

#[derive(Cli)]
#[command(
    name = "validate-grammar",
    about = "Run the generated css parser on the provided source"
)]
struct ValidationArgs {
    /// the name of the parser rule to evaluate
    rule: String,

    /// parse contents of file at this path
    source_path: String,
}

fn main() {
    let rule_map: HashMap<&str, Rule> = collection! {
        "css"               => Rule::Css,
        "rule"              => Rule::Rule,
        "selector"          => Rule::Selector,
        "complexselector"   => Rule::ComplexSelector,
        "combinator"        => Rule::Combinator,
        "nextsibling"       => Rule::NextSibling,
        "child"             => Rule::Child,
        "column"            => Rule::Column,
        "subsequentsibling" => Rule::SubsequentSibling,
        "descendent"        => Rule::Descendent,
        "namespace"         => Rule::Namespace,
        "compoundselector"  => Rule::CompoundSelector,
        "simpleselector"    => Rule::SimpleSelector,
        "basicselector"     => Rule::BasicSelector,
        "typeselector"      => Rule::TypeSelector,
        "classselector"     => Rule::ClassSelector,
        "idselector"        => Rule::IdSelector,
        "universalselector" => Rule::UniversalSelector,
        "property"          => Rule::Property,
        "value"             => Rule::Value,
        "number"            => Rule::Number,
        "int"               => Rule::Int,
        "float"             => Rule::Float,
        "unit"              => Rule::Unit,
        "string"            => Rule::String,
        "identifier"        => Rule::Identifier,
    };

    let arguments = ValidationArgs::parse();

    let match_rule = rule_map
        .get(arguments.rule.to_lowercase().as_str())
        .expect(format!("Could not parse rule type '{}'!", arguments.rule).as_str());

    let css_source: String;
    match read_to_string(&arguments.source_path) {
        Ok(contents) => css_source = contents,
        Err(_) => panic!("Failed to read '{}'!", arguments.source_path),
    }

    let parsed_result = CssParser::parse(*match_rule, &css_source);
    let grammar_rules: FlatPairs<'_, Rule>;
    match parsed_result {
        Ok(pairs) => grammar_rules = pairs.flatten(),
        Err(err) => {
            println!("{err}");
            return;
        }
    }

    for _rule in grammar_rules {}
}
