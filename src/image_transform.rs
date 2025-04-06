use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct Transformations {
    resize: Resize,
    crop: Crop,
    rotate: Rotate,
    format: Format,
    filters: Filters,
}

#[derive(Deserialize,Serialize)]
pub struct Resize {
    width: i32,
    height: i32,
}

#[derive(Deserialize,Serialize)]
pub struct Crop {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
}

#[derive(Deserialize,Serialize)]
pub struct Rotate(i32);

#[derive(Deserialize,Serialize)]
pub struct Format(String);

#[derive(Deserialize,Serialize)]
pub struct Filters {
    grayscale: bool,
    sepia: bool,
}
