use pest::iterators::Pair;
use pest_consume::{match_nodes, Error, Parser};

use std::collections::HashMap;

use crate::values::*;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[derive(Parser)]
#[grammar = "css.pest"]
pub struct CssParser;

// fn dummy(node: Node) {
//     node.
// }

#[pest_consume::parser]
impl CssParser {
    pub fn Css(input: Node) -> Result<Stylesheet> {
        match_nodes!(input.into_children();
            [Rule(rules).., EOI(_)] => Ok(rules.collect())
        )
    }

    pub fn Rule(input: Node) -> Result<RuleSet> {
        match_nodes!(input.into_children();
            [Selector(sel), PropertyList(props)] => Ok(RuleSet{
                selectors: sel,
                properties: props,
            })
        )
    }

    pub fn Selector(input: Node) -> Result<Selector> {
        match_nodes!(input.into_children();
            [CompoundSelector(sel)] => Ok(sel)
        )
    }

    pub fn CompoundSelector(input: Node) -> Result<CompoundSelector> {
        match_nodes!(input.into_children();
            [ComplexSelector(first), ComplexSelector(selectors)..] => Ok((first, selectors.collect()))
        )
    }

    pub fn ComplexSelector(input: Node) -> Result<ComplexSelector> {
        match_nodes!(input.into_children();
            [SimpleSelector(first), Combination(comb)..] => Ok((first, comb.collect()))
        )
    }

    pub fn Combination(input: Node) -> Result<Combination> {
        match_nodes!(input.into_children();
            [Combinator(comb), SimpleSelector(sel)] => Ok((comb, sel))
        )
    }

    pub fn Combinator(input: Node) -> Result<Combinator> {
        Ok(input.as_str().parse::<Combinator>().unwrap())
    }

    pub fn SimpleSelector(input: Node) -> Result<SimpleSelector> {
        match_nodes!(input.clone().into_children();
            [BasicSelector(sels)..] => {
                let mut selector = SimpleSelector::new();

                for sel in sels {
                    match sel {
                        BasicSelector::Id(id) => {
                            if selector.id.is_some() {
                                return Err(input.error("BasicSelectors cannot have more than one IdSelector!"));
                            }
                            selector.id = Some(id);
                        },
                        BasicSelector::Class(class) => {
                            if ! selector.classes.contains(&class) {
                                selector.classes.push(class);
                            }
                        },
                        BasicSelector::Type(type_) => {
                            if selector.name.is_some() {
                                return Err(input.error("BasicSelectors cannot have more than one TypeSelector!"));
                            }
                            selector.name = Some(type_);
                        },
                        BasicSelector::Universal => selector.universal = true,
                    }
                }

                Ok(selector)
            }
        )
    }

    pub fn BasicSelector(input: Node) -> Result<BasicSelector> {
        match_nodes!(input.into_children();
            [IdSelector(sel)] => Ok(BasicSelector::Id(sel)),
            [ClassSelector(sel)] => Ok(BasicSelector::Class(sel)),
            [TypeSelector(sel)] => Ok(BasicSelector::Type(sel)),
            [UniversalSelector] => Ok(BasicSelector::Universal),
        )
    }

    pub fn IdSelector(input: Node) -> Result<String> {
        match_nodes!(input.into_children();
            [BasicSelector(sel)] => Ok(sel.inner().unwrap())
        )
    }

    pub fn ClassSelector(input: Node) -> Result<String> {
        match_nodes!(input.into_children();
            [BasicSelector(sel)] => Ok(sel.inner().unwrap())
        )
    }

    pub fn TypeSelector(input: Node) -> Result<String> {
        match_nodes!(input.into_children();
            [Identifier(ident)] => Ok(ident)
        )
    }

    pub fn UniversalSelector(input: Node) -> Result<()> {
        Ok(())
    }

    pub fn PropertyList(input: Node) -> Result<HashMap<String, Vec<ValueType>>> {
        let mut property_list: HashMap<String, Vec<ValueType>> = HashMap::new();

        match_nodes!(input.into_children();
            [Property(properties)..] => {
                for (key, value_list) in properties {
                    property_list.insert(key, value_list);
                }
            }
        );

        Ok(property_list)
    }

    pub fn Property(input: Node) -> Result<(String, Vec<ValueType>)> {
        // println!(".");
        let mut result: Result<(String, Vec<ValueType>)> = Err(input.error(""));
        match_nodes!(input.into_children();
            [Identifier(ident), ValueList(values)] => {
                result = Ok((ident, values));
            }
        );
        // println!(".");
        result
    }

    pub fn ValueList(input: Node) -> Result<Vec<ValueType>> {
        let mut value_list = Vec::new();
        match_nodes!(input.into_children();
            [Value(values)..] => {
                value_list.extend(values);
            }
        );
        Ok(value_list)
    }

    pub fn Value(input: Node) -> Result<ValueType> {
        match_nodes!(input.into_children();
            [Function(f)] => Ok(ValueType::Function(f)),
            [Number(n)] => Ok(ValueType::Number(n)),
            [Identifier(ident)] => Ok(ValueType::Identifier(ident)),
            [String(s)] => Ok(ValueType::String(s)),
            [Hex(h)] => Ok(ValueType::Color(h)),
        )
    }

    pub fn Function(input: Node) -> Result<Function> {
        match_nodes!(input.into_children();
            [Identifier(ident), ValueList(values)] => Ok((ident, values)),
        )
    }

    pub fn Hex(input: Node) -> Result<Color> {
        let raw_num = input.as_str();
        Ok([
            u8::from_str_radix(&raw_num[1..3], 16).unwrap(),
            u8::from_str_radix(&raw_num[3..5], 16).unwrap(),
            u8::from_str_radix(&raw_num[5..7], 16).unwrap(),
        ])
    }

    pub fn Number(input: Node) -> Result<Number> {
        match_nodes!(input.clone().into_children();
            [Sign(sign),    Float(value),   Unit(unit)] => Ok((sign as f32 * value, Some(unit))),
            [               Float(value),   Unit(unit)] => Ok((value, Some(unit))),
            [Sign(sign),    Int(value),     Unit(unit)] => Ok(((sign * value) as f32, Some(unit))),
            [               Int(value),     Unit(unit)] => Ok((value as f32, Some(unit))),

            [Sign(sign),    Float(value)]   => Ok((sign as f32 * value, None)),
            [               Float(value)]   => Ok((value, None)),
            [Sign(sign),    Int(value)]     => Ok(((sign * value) as f32, None)),
            [               Int(value)]     => Ok((value as f32, None)),
        )
    }

    pub fn Unit(input: Node) -> Result<Unit> {
        Ok(input.as_str().parse::<Unit>().unwrap())
    }

    pub fn Float(input: Node) -> Result<f32> {
        Ok(input.as_str().parse::<f32>().unwrap())
    }

    pub fn Int(input: Node) -> Result<i32> {
        match input.clone().as_str().parse::<i32>() {
            Ok(num) => Ok(num),
            Err(_) => Err(input.error(format!("Failed to convert '{}' into an i32!", input.clone().as_str()))),
        }
    }

    pub fn Sign(input: Node) -> Result<i32> {
        match input.as_str() {
            "-" => Ok(-1),
            _ => Ok(1),
        }
    }

    pub fn String(input: Node) -> Result<String> {
        Ok(input.as_str().to_string())
    }

    pub fn Identifier(input: Node) -> Result<String> {
        Ok(input.as_str().to_string())
    }

    pub fn EOI(input: Node) -> Result<()> {
        Ok(())
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
