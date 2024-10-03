#![allow(warnings)]

use pest::{Parser, iterators::Pair, Token, Position, Span};

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

    let rule_raw = std::env::args().nth(1).expect("No rule provided!");
    let mut input = std::env::args().nth(2).expect("No source provided!");

    let rule = rule_map.get(rule_raw.to_lowercase().as_str())
        .expect(format!("Could not parse rule type '{}'!", rule_raw).as_str())
        ;

    if input.eq("-f") {
        let filepath = std::env::args().nth(3).expect("No file provided!");
        let result = read_to_string(filepath);
        match result {
            Ok(contents) => input = contents,
            Err(_) => todo!(),
        }
    }

    let mut parsed = CssParser::parse(*rule, input.as_str())
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
    
    let big_rules = vec![Rule::Css, Rule::Rule, Rule::PropertyList, Rule::Selector];

    for (depth, token, source) in tokens.iter().rev() {
        let indent = std::iter::repeat(" ").take(2 * depth).collect::<String>();
        let token_type = format!("{token:?}");
        let offset = 25 - (2 * depth);

        if !big_rules.contains(token) {
            println!("{indent}- {token_type:<offset$}: '{source}'");
        }
        else {
            println!("{indent}- {token_type:<offset$}");
        }
    }

}
