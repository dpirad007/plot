mod line;

#[derive(Clone, Debug)]
pub struct Graph {
    pub name: String,
    // pub size: usize,
    pub points: Vec<Point>,
    pub color: String,
    // pub x_range: f64,
    // pub y_range: f64,
    // pub x_min: f64,
    // pub y_min: f64,
}

#[derive(Clone, Debug, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
