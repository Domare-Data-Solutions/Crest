
use pest::{Parser, Token, Position};
use clap::{arg, command, Parser as Cli};

use std::collections::HashMap;
use std::fs::read_to_string;

use peacock_crest::{CssParser, Rule};

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
#[command(name = "validate-grammar", about = "Run the generated css parser on the provided source")]
struct ValidationArgs {
    /// use `source` as a filepath to read instead of raw text
    #[arg(short, long)]
    file: bool,

    /// the name of the parser rule to evaluate
    rule: String,

    /// default: raw-text to parse ; --file mode: read file at this path, parse contents
    source: String,
}

fn main() {
    let rule_map: HashMap<&str, Rule> = collection!{
        "css"               => Rule::Css,
        "rule"              => Rule::Rule,
        "selector"          => Rule::Selector,
        "complexselector"   => Rule::ComplexSelector,
        "combination"       => Rule::Combination,
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
        "propertylist"      => Rule::PropertyList,
        "property"          => Rule::Property,
        "propertyassign"    => Rule::PropertyAssign,
        "propertystatement" => Rule::PropertyStatement,
        "value"             => Rule::Value,
        "number"            => Rule::Number,
        "withunit"          => Rule::WithUnit,
        "withoutunit"       => Rule::WithoutUnit,
        "int"               => Rule::Int,
        "float"             => Rule::Float,
        "unit"              => Rule::Unit,
        "string"            => Rule::String,
        "identifier"        => Rule::Identifier,
        "whitespace"        => Rule::Whitespace,
    };

    let arguments = ValidationArgs::parse();

    let source: String;

    let rule = rule_map.get(arguments.rule.to_lowercase().as_str())
        .expect(format!("Could not parse rule type '{}'!", arguments.rule).as_str())
        ;

    if arguments.file {
        let filepath = std::env::args().nth(3).expect("No file provided!");
        let result = read_to_string(filepath);
        match result {
            Ok(contents) => source = contents,
            Err(_) => panic!("Failed to read '{}'!", arguments.source),
        }
    }
    else {
        source = arguments.source;
    }

    let parsed = CssParser::parse(*rule, &source)
        .expect("Failed to parse provided source")
        .flatten()
        ;

    let mut stack: Vec<(Rule, Position)> = Vec::new();
    let mut tokens: Vec<(usize, Rule, String)> = Vec::new();
    let mut depth = 0usize;
    
    for token in parsed.tokens() {
        match token {
            Token::Start { rule, pos } => {
                // if !vec![]
                depth += 1;
                stack.push((rule.clone(), pos));
            },
            Token::End { rule, pos } => {
                let (start_rule, start_pos) = stack.pop().unwrap();
                assert_eq!(start_rule, rule);
                tokens.push((depth, rule, start_pos.span(&pos).as_str().to_string()));
                depth -= 1;
            },
        }
    }
    
    let no_print: Vec<Rule> = collection![Rule::Css, Rule::Rule, Rule::PropertyList, Rule::Selector];

    for (depth, token, source) in tokens.iter().rev() {
        let indent = std::iter::repeat(" ").take(2 * depth).collect::<String>();
        let token_type = format!("{token:?}");
        let offset = 25 - (2 * depth);

        if !no_print.contains(token) {
            println!("{indent}- {token_type:<offset$}: '{source}'");
        }
        else {
            println!("{indent}- {token_type:<offset$}");
        }
    }

}
