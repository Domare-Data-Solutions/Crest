
use pest::{iterators::FlatPairs, Parser, Position, Token};
use clap::{arg, command, Parser as Cli};

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
#[command(name = "validate-grammar", about = "Run the generated css parser on the provided source")]
struct ValidationArgs {
    /// use `source` as a filepath to read instead of raw text
    #[arg(short, long)]
    file: bool,

    /// do not print the children of matches
    #[arg(short, long)]
    no_print_children: bool,

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

    let source: String;

    let match_rule = rule_map.get(arguments.rule.to_lowercase().as_str())
        .expect(format!("Could not parse rule type '{}'!", arguments.rule).as_str())
        ;

    if arguments.file {
        let result = read_to_string(&arguments.source);
        match result {
            Ok(contents) => source = contents,
            Err(_) => panic!("Failed to read '{}'!", arguments.source),
        }
    }
    else {
        source = arguments.source;
    }

    let parsed_result = CssParser::parse(*match_rule, &source);
    let parsed: FlatPairs<'_, Rule>;
    match parsed_result {
        Ok(pairs) => parsed = pairs.flatten(),
        Err(err) => {
            println!("{err}");
            return;
        },
    }

    let mut stack: Vec<(Rule, Position)> = Vec::new();
    let mut tokens: Vec<(usize, Rule, String)> = Vec::new();
    let mut depth = 0usize;

    // let mut match_depth: Option<usize> = None;
    
    for token in parsed.tokens() {
        match token {
            Token::Start { rule, pos } => {
                // if !vec![]
                // if rule == *match_rule {
                //     match_depth = Some(depth);
                // }
                depth += 1;
                stack.push((rule.clone(), pos));
            },
            Token::End { rule, pos } => {
                let (start_rule, start_pos) = stack.pop().unwrap();
                assert_eq!(start_rule, rule);
                depth -= 1;
                // if match_depth.is_some_and(|x| depth == x) {
                //     match_depth = None;
                // }
                // if !(arguments.no_print_children && match_depth.is_some_and(|x| depth > x)) {
                    tokens.push((depth, rule, start_pos.span(&pos).as_str().to_string()));
                // }

            },
        }
    }
    
    let no_print: Vec<Rule> = collection![Rule::Css, Rule::Rule, Rule::Property, Rule::Selector];

    for (depth, token, source) in tokens.iter().rev() {
        let indent = std::iter::repeat(" ").take(2 * depth).collect::<String>();
        let token_type = format!("{token:?}");
        let offset = 28 - (2 * depth);

        if !no_print.contains(token) {
            println!("{indent}- {token_type:<offset$}: '{source}'");
        }
        else {
            println!("{indent}- {token_type:<offset$}");
        }
    }

}
