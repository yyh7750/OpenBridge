use serde::{Deserialize, Serialize};
use super::instrument::InstrumentFields;

#[derive(Debug, Clone, Serialize, Deserialize)] // Clone을 추가해 Arc 복사가 가능하도록 설정
#[serde(rename_all = "camelCase")]
pub struct Compass {
    instrument_fields: InstrumentFields,
    heading_advices: HeadingAdvices,
}

impl Compass {
    pub fn new() -> Self {
        Self {
            instrument_fields: InstrumentFields::new(),
            heading_advices: HeadingAdvices::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HeadingAdvice {
    min_angle: i32,
    max_angle: i32,
    _type: String,
    hinted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadingAdvices {
    heading: i32,
    course_over_ground: i32,
    heading_advices: Vec<HeadingAdvice>,
}

impl HeadingAdvices {
    pub fn new() -> Self {
        Self {
            heading: 0,
            course_over_ground: 0,
            heading_advices: vec![HeadingAdvice {
                min_angle: 0,
                max_angle: 0,
                _type: "advice".to_owned(),
                hinted: false,
            }],
        }
    }
}
