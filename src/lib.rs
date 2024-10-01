
//! # Crest
//! 
//! Crest is [Peacock](#)'s core library for parsing css files.
//! While Crest is intended for use by Peacock, it is designed
//! to be usable for other projects as well.
//! 
//! For more information on Peacock, [click here](#)!
//! 

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub css
);

pub mod ast;
