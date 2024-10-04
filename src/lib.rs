#![allow(warnings)]

//! # Crest
//! 
//! Crest is [Peacock](#)'s core library for parsing css files.
//! While Crest is intended for use by Peacock, it is designed
//! to be usable for other projects as well.
//! 
//! For more information on Peacock, [click here](#)!
//! 

pub mod parse;

// use pest::Token;

use parse::Rule;

#[derive(Debug)]
pub enum Error {
    IncorrectRuleType(Rule, Rule),
}

pub struct PropertyList(std::collections::HashMap<String, Vec<String>>);

pub struct Selector {
    name: Option<String>,
    id: Option<String>,
    classes: Vec<String>,
    universal: bool,
}

pub struct RuleSet {
    selectors: std::rc::Rc<[Selector]>,
    properties: PropertyList,
}

// ========== IMPLS ===========

fn find_descendent_type<'a>(ancestor: pest::iterators::Pair<'a, Rule>, stack: &mut Vec<Rule>, stack_depth: usize) -> Vec<pest::iterators::Pair<'a, Rule>> {
    assert!(stack_depth < stack.len());
    let current_rule_search = stack[stack_depth];
    let mut results: Vec<pest::iterators::Pair<'a, Rule>> = Vec::new();
    for pair in ancestor.into_inner() {
        if pair.as_rule() == current_rule_search {
            results.append(&mut find_descendent_type(pair, stack, stack_depth + 1))
        }
    }
    results
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// impl<'a> TryFrom<pest::iterators::Pair<'a, Rule>> for Selector {
//     type Error = Error;

//     fn try_from(value: pest::iterators::Pair<'a, Rule>) -> Result<Self, Self::Error> {
//         assert_eq!(value.as_rule(), Rule::Selector);
        
//         if value.as_rule() == Rule::Selector && !value.as_str().eq("*") {
//             let mut name_stack: Vec<Rule> = vec![Rule::CompoundSelector, Rule::SimpleSelector, Rule::BasicSelector];
//             let names = find_descendent_type(value.clone(), &mut name_stack);
//             let name = match name_opt {
//                 Some(name) => Some(name.as_str().to_string()),
//                 None => None
//             };

//             let mut id_stack: Vec<Rule> = vec![Rule::CompoundSelector, Rule::SimpleSelector, Rule::BasicSelector, Rule::IdSelector];
//             let id_opt = find_descendent_type(value.clone(), &mut id_stack);
//             todo!()
//             // Ok(Self{
//             //     name,

//             // })
//         }
//         else if value.as_rule() == Rule::Selector {
//             Ok(Self{
//                 name: None,
//                 id: None,
//                 classes: vec![],
//                 universal: true,
//             })
//         }
//         else {
//             Err(Self::Error::IncorrectRuleType(Rule::Selector, value.as_rule()))
//         }
//     }
// }
