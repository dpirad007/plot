use super::{Graph, Point};
// use crate::read_csv_rust;
use tera::{Context, Tera};
use wasm_bindgen::prelude::*;

impl Graph {
    pub fn new(name: String, color: String) -> Self {
        Graph {
            name,
            points: Vec::new(),
            color,
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        self.points.push(Point { x, y });
    }

    pub fn draw_svg(
        &self,
        width: usize,
        height: usize,
        viewbox_height: usize,
        viewbox_width: usize,
    ) -> String {
        let mut context = Context::new();

        //hardset the padding around the graph
        let padding = 50;

        //ensure the viewbox is as per input
        let width = width - padding * 2;
        let height = height - padding * 2;

        let max_x = self
            .points
            .iter()
            .map(|point| point.x)
            .fold(0. / 0., f64::max);

        let max_y = self
            .points
            .iter()
            .map(|point| point.y)
            .fold(0. / 0., f64::max);

        let path = self
            .points
            .iter()
            .map(|val| Point {
                x: (val.x / max_x * width as f64) + padding as f64,
                y: (val.y / max_y * (height as f64 * -1.0)) + (padding + height) as f64,
            })
            .enumerate()
            .map(|(i, point)| {
                if i == 0 {
                    format!("M {} {}", point.x, point.y)
                } else {
                    format!("L {} {}", point.x, point.y)
                }
            })
            .collect::<Vec<String>>()
            .join(" ");

        context.insert("name", &self.name);
        context.insert("viewbox_width", &viewbox_width);
        context.insert("viewbox_height", &viewbox_height);
        context.insert("width", &width);
        context.insert("height", &height);
        context.insert("padding", &padding);
        context.insert("color", &self.color);
        context.insert("path", &path);
        context.insert("lines", &5);

        Tera::one_off(include_str!("line.svg"), &context, true).expect("Could not draw graph")
    }
}

pub fn generate_graph(chart_name: String, chart_color: String) -> Graph {
    let mut graph = Graph::new(chart_name.into(), chart_color.into());
    graph.add_point(0.0, 0.0);
    graph.add_point(2.0, 3.0);
    graph.add_point(3.0, 2.5);
    graph.add_point(4.0, 6.0);
    graph.add_point(5.0, 3.0);
    graph
}

#[wasm_bindgen(js_name = plotLine)]
pub fn display_graph(name: JsValue, color: JsValue) -> String {
    let chart_name = serde_wasm_bindgen::from_value(name).unwrap();
    let chart_color = serde_wasm_bindgen::from_value(color).unwrap();

    let graph = generate_graph(chart_name, chart_color);

    graph.draw_svg(800, 400, 700, 250)
}
