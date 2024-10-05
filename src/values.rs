#[derive(Debug)]
pub enum Error {
    ConversionError(String),
}

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

#[derive(Debug, Clone)]
pub enum Unit {
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

pub type Function = (String, Vec<ValueType>);
pub type Number = (f32, Unit);
pub type Color = [u8; 3];

#[derive(Debug, Clone)]
pub enum ValueType {
    Identifier(String),
    Number(Number),
    String(String),
    Color(Color),
    Function(Function),
}

// ========== IMPLS ===========

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl TryFrom<ValueType> for DisplayOption {
    type Error = Error;

    fn try_from(value: ValueType) -> Result<Self, Self::Error> {
        match value {
            ValueType::Identifier(display) => match display.as_str() {
                "none" => Ok(DisplayOption::None),
                "block" => Ok(DisplayOption::Block),
                "inline" => Ok(DisplayOption::Inline),
                "flow" => Ok(DisplayOption::Flow),
                _ => Err(Self::Error::ConversionError(format!(
                    "invalid DisplayOption value: {display}"
                ))),
            },
            _ => Err(Self::Error::ConversionError(format!(
                "unknown DisplayOption value type: {value:?}"
            ))),
        }
    }
}

impl TryFrom<(ValueType, ValueType)> for Display {
    type Error = Error;

    fn try_from(value: (ValueType, ValueType)) -> Result<Self, Self::Error> {
        let inner_result = DisplayOption::try_from(value.0);
        let outer_result = DisplayOption::try_from(value.1);

        if inner_result.is_ok() && outer_result.is_ok() {
            Ok(Display {
                inner: inner_result.unwrap(),
                outer: outer_result.unwrap(),
            })
        } else if inner_result.is_err() {
            Err(inner_result.unwrap_err())
        } else {
            Err(outer_result.unwrap_err())
        }
    }
}
