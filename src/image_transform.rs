pub struct Transformations {
    resize: Resize,
    crop: Crop,
    rotate: Rotate,
    format: Format,
    filters: Filters,
}

pub struct Resize {
    width: i32,
    height: i32,
}

pub struct Crop {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
}

pub struct Rotate(i32);

pub struct Format(String);

pub struct Filters {
    grayscale: bool,
    sepia: bool,
}
