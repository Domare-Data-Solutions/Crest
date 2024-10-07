use pest::iterators::Pair;
use strum_macros::EnumString;

use super::parse::Rule;

#[derive(Default)]
pub enum Position {
    #[default]
    Static,
    Relative,
    Absolute,
}

#[derive(Debug, Default)]
pub enum DisplayOption {
    None,
    #[default]
    Block,
    Inline,
    Flow,
    // todo: inline-block, flex, grid
}
#[derive(Default)]
pub struct Display {
    pub outer: DisplayOption,
    pub inner: DisplayOption,
}

#[derive(Default)]
pub enum Visibility {
    #[default]
    Visible,
    Hidden,
}

#[derive(Debug, Clone, EnumString)]
pub enum Unit {
    #[strum(serialize = "cm")]
    Cm,
    #[strum(serialize = "mm")]
    Mm,
    #[strum(serialize = "in")]
    In,
    #[strum(serialize = "px")]
    Px,
    #[strum(serialize = "pt")]
    Pt,
    #[strum(serialize = "pc")]
    Pc,

    #[strum(serialize = "em")]
    Em,
    #[strum(serialize = "ex")]
    Ex,
    #[strum(serialize = "ch")]
    Ch,
    #[strum(serialize = "rem")]
    Rem,
    #[strum(serialize = "vw")]
    VW,
    #[strum(serialize = "vh")]
    VH,
    #[strum(serialize = "vmin")]
    VMin,
    #[strum(serialize = "vmax")]
    VMax,
    #[strum(serialize = "%")]
    Percent,
}

pub type Function = (String, Vec<ValueType>);
pub type Number = (f32, Option<Unit>);
pub type Color = [u8; 3];

#[derive(Debug, Clone)]
pub enum ValueType {
    Identifier(String),
    Number(Number),
    String(String),
    Color(Color),
    Function(Function),
}

#[derive(Debug, Clone, EnumString)]
pub enum Combinator {
    None,
    #[strum(serialize = "+")]
    NextSibling,
    #[strum(serialize = ">")]
    Child,
    #[strum(serialize = "||")]
    Column,
    #[strum(serialize = "~")]
    SubsequentSibling,
    #[strum(serialize = "|")]
    Namespace,
    #[strum(serialize = " ")]
    Descendent,
}

pub enum BasicSelector {
    Id(String),
    Class(String),
    Type(String),
    Universal,
}

pub type PropertyList = std::collections::HashMap<String, Vec<ValueType>>;

#[derive(Clone, Debug)]
pub struct SimpleSelector {
    pub name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub universal: bool,
}

pub type Combination = (Combinator, SimpleSelector);
pub type ComplexSelector = (SimpleSelector, Vec<Combination>);
pub type CompoundSelector = (ComplexSelector, Vec<ComplexSelector>);

pub type Selector = CompoundSelector;

#[derive(Clone, Debug)]
pub struct RuleSet {
    pub selectors: Selector,
    pub properties: PropertyList,
}

pub type Stylesheet = Vec<RuleSet>;

// ========== IMPLS ===========

impl BasicSelector {
    pub fn inner(&self) -> Option<String> {
        match self {
            BasicSelector::Id(string) => Some(string.clone()),
            BasicSelector::Class(string) => Some(string.clone()),
            BasicSelector::Type(string) => Some(string.clone()),
            _ => None,
        }
    }
}

impl SimpleSelector {
    pub fn new() -> Self {
        Self {
            name: None,
            id: None,
            classes: Vec::new(),
            universal: false,
        }
    }
}
