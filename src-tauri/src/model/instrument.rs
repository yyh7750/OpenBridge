use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
enum InstrumentFieldSize {
    Default,
    Small,
    Large,
    Enhanced,
    LargeEnhanced,
}

impl fmt::Display for InstrumentFieldSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: &str = match self {
            InstrumentFieldSize::Default => "",
            InstrumentFieldSize::Small => "small",
            InstrumentFieldSize::Large => "large",
            InstrumentFieldSize::Enhanced => "enhanced",
            InstrumentFieldSize::LargeEnhanced => "large-enhanced",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InstrumentField {
    setpoint: i32,
    has_setpoint: bool,
    value: f64,
    degree: bool,
    tag: String,
    unit: String,
    source: String,
    has_source: bool,
    size: String,
    max_digits: i32,
    fraction_digits: i32,
}

impl InstrumentField {
    pub fn new() -> Self {
        Self {
            setpoint: 0,
            has_setpoint: true,
            value: 0.0,
            degree: true,
            tag: "HDG".to_owned(),
            unit: "/min".to_owned(),
            source: "SRC".to_owned(),
            has_source: true,
            size: InstrumentFieldSize::Default.to_string(),
            max_digits: 3,
            fraction_digits: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentFields {
    hdg: InstrumentField,
    cog: InstrumentField,
    rot: InstrumentField,
    wind: InstrumentField,
    current: InstrumentField,
}

impl InstrumentFields {
    pub fn new() -> Self {
        Self {
            hdg: InstrumentField::new(),
            cog: InstrumentField::new(),
            rot: InstrumentField::new(),
            wind: InstrumentField::new(),
            current: InstrumentField::new(),
        }
    }
}