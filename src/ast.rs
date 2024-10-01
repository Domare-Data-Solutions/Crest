#![allow(dead_code)]

pub enum Unit {
    Default,

    Cm,
    Mm,
    In,
    Px,
    Pt,
    Pc,

    Em,
    Ex,
    Ch,
    Rem,
    VW,
    VH,
    VMin,
    VMax,
    Percent,
}

pub type Identifier = String;
pub type Value = Option<(String, Unit)>;
pub type Property = (Identifier, Value);

pub struct PropertySet {
    pub selectors: Vec<Selector>,
    pub properties: Vec<Property>,
}

pub enum BasicSelector {
    Type(String),
    Class(String),
    Id(String),
    Universal,
}

pub enum SimpleSelector {
    Basic(BasicSelector),
    // TODO: Implement
    // Attribute(Identifier, String, Vec<Identifier>),
    // PseudoClass,
    // PseudoElement,
}

pub type CompoundSelector = Vec<SimpleSelector>;

pub enum Combinator {
    NextSibling(CompoundSelector),
    Child(CompoundSelector),
    Column(CompoundSelector),
    SubsequentSibling(CompoundSelector),
    Descendent(CompoundSelector),
    Namespace(CompoundSelector),
}

pub type ComplexSelector = (SimpleSelector, Vec<Combinator>);

pub enum Selector {
    Simple(SimpleSelector),
    Compound(CompoundSelector),
    Complex(ComplexSelector),
    // TODO: Implement
    // Relative(?),
}

impl PropertySet {
    pub fn new(selectors: Vec<Selector>, properties: Vec<Property>) -> Self {
        Self{
            selectors,
            properties,
        }
    }
}

impl From<&str> for Unit {
    fn from(value: &str) -> Self {
        if value.ends_with("ch") { Self::Ch }
        else if value.ends_with("mm") { Self::Mm }
        else if value.ends_with("in") { Self::In }
        else if value.ends_with("px") { Self::Px }
        else if value.ends_with("pt") { Self::Pt }
        else if value.ends_with("pc") { Self::Pc }

        else if value.ends_with("em") { Self::Em }
        else if value.ends_with("ex") { Self::Ex }
        else if value.ends_with("ch") { Self::Ch }
        else if value.ends_with("rem") { Self::Rem }
        else if value.ends_with("vw") { Self::VW }
        else if value.ends_with("vh") { Self::VH }
        else if value.ends_with("vmin") { Self::VMin }
        else if value.ends_with("vmax") { Self::VMax }
        else if value.ends_with("%") { Self::Percent }

        else { Self::Default }
    }
}
