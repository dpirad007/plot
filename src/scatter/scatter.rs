use super::Graph;
use super::Point;
use crate::read_csv_rust;
use std::iter::Iterator;
use tera::{Context, Tera};
use wasm_bindgen::prelude::*;

impl Graph {
    pub fn new(name: String, color: String) -> Self {
        Graph {
            name,
            size: 0,
            points: Vec::new(),
            color,
            x_range: 0.,
            y_range: 0.,
            x_min: 0.,
            y_min: 0.,
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push(Point { x, y });
    }

    pub fn draw_svg(
        &self,
        width: usize,
        height: usize,
        padding: usize,
        path: Vec<Point>,
    ) -> String {
        let mut context = Context::new();

        let mut p: Vec<(f64, f64)> = Vec::new();

        for point in path {
            p.push((point.x, point.y));
        }

        //ensure the viewbox is as per input
        context.insert("name", &self.name);
        context.insert("width", &width);
        context.insert("height", &height);
        context.insert("padding", &padding);
        context.insert("path", &p);
        // context.insert("centers", &centers);
        context.insert("x_range", &self.x_range);
        context.insert("y_range", &self.y_range);
        context.insert("x_min", &self.x_min);
        context.insert("y_min", &self.y_min);
        context.insert("color", &self.color);
        context.insert("lines", &10);

        Tera::one_off(include_str!("scatter.svg"), &context, true).expect("Could not draw graph")
    }
}

pub fn generate_graph(xs: Vec<f64>, ys: Vec<f64>, title: String) -> Graph {
    let mut graph = Graph::new(title.into(), "#35fcf6".into());
    graph.size = xs.len();
    for i in 0..graph.size {
        graph.add_point(xs[i], ys[i]);
    }
    return graph;
}

#[wasm_bindgen(js_name = plotScatter)]
pub async fn fit_draw(
    csv_data: web_sys::File,
    title: JsValue,
    width: usize,
    height: usize,
    padding: usize,
) -> String {
    let chart_name = serde_wasm_bindgen::from_value(title).unwrap();
    let data: Vec<f64> = read_csv_rust(csv_data).await;
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    let mut tuples: Vec<(f64, f64)> = Vec::new();

    for i in 0..data.len() {
        if (i % 2) == 1 {
            tuples.push((data[i - 1], data[i]));
        }
    }

    for i in 0..tuples.len() {
        xs.push(tuples[i].0);
        ys.push(tuples[i].1);
    }

    let mut graph = generate_graph(xs, ys, chart_name);

    let width = width - padding * 2;
    let height = height - padding * 2;
    let x_max = graph
        .points
        .iter()
        .map(|point| point.x)
        .fold(0. / 0., f64::max);
    let x_min = graph
        .points
        .iter()
        .map(|point| point.x)
        .fold(0. / 0., f64::min);
    let y_max = graph
        .points
        .iter()
        .map(|point| point.y)
        .fold(0. / 0., f64::max);
    let y_min = graph
        .points
        .iter()
        .map(|point| point.y)
        .fold(0. / 0., f64::min);

    graph.x_min = (x_min - 1.0).round();
    graph.y_min = (y_min - 1.0).round();

    graph.x_range = (x_max + 1.0).round() - graph.x_min;
    graph.y_range = (y_max + 1.0).round() - graph.y_min;

    let path = graph
        .points
        .iter()
        .map(|val| Point {
            x: ((val.x - graph.x_min) / graph.x_range * width as f64) + padding as f64,
            y: ((val.y - graph.y_min) / graph.y_range * (height as f64 * -1.0))
                + (padding + height) as f64,
        })
        .collect();

    let out = graph.draw_svg(width, height, padding, path);
    out
}
