use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Debug, Deserialize,Serialize)]
pub struct Transformation {
    pub resize: Resize,
    pub crop: Crop,
    pub rotate: Rotate,
    pub format: Format,
    pub filters: Filters,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct Resize {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct Crop {
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct Rotate(i32);

#[derive(Debug, Deserialize,Serialize)]
pub struct Format(String);

#[derive(Debug, Deserialize,Serialize)]
pub struct Filters {
    pub grayscale: bool,
    pub sepia: bool,
}

impl ToString for Transformation {
    fn to_string(&self) -> String {
        format!("{:?}", self)    
    }
}
