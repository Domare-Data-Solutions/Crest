
//! # Crest
//! 
//! Crest is [Peacock](#)'s core library for parsing css files.
//! While Crest is intended for use by Peacock, it is designed
//! to be usable for other projects as well.
//! 
//! For more information on Peacock, [click here](#)!
//! 

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "css.pest"]
pub struct CssParser;

pub mod ast;
