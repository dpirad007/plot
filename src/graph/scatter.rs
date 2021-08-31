use super::{Graph, Point};
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

    pub fn draw_svg(&self, width: usize, height: usize) -> String {
        let mut context = Context::new();

        //hardset the padding around the graph
        let padding = 50;

        //ensure the viewbox is as per input
        let width = width - padding * 2;
        let height = height - padding * 2;

        context.insert("name", &self.name);
        context.insert("width", &width);
        context.insert("height", &height);
        context.insert("padding", &padding);
        context.insert("color", &self.color);
        Tera::one_off(include_str!("graph.svg"), &context, true).expect("Could not draw graph")
    }
}

pub fn generate_graph(chart_name: String, chart_color: String) -> Graph {
    let graph = Graph::new(chart_name.into(), chart_color.into());
    graph
}

#[wasm_bindgen(js_name = plotGraph)]
pub fn display_graph(name: JsValue, color: JsValue) -> String {
    let chart_name = serde_wasm_bindgen::from_value(name).unwrap();
    let chart_color = serde_wasm_bindgen::from_value(color).unwrap();
    let graph = generate_graph(chart_name, chart_color);

    graph.draw_svg(100, 100)
}
