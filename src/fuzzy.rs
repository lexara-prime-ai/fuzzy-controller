use std::{collections::HashMap, fmt::Display, hash::Hash, ops::Range, rc::Rc};

use egui_macroquad::egui::Context;

use crate::{mie::InputType, ui::Graph};
pub struct Fuzzy<V>
where
    V: Eq + Hash + Copy + Display,
{
    members: usize,
    pub functions: Rc<HashMap<V, Box<dyn Fn(f32) -> f32>>>,
    range: Range<f32>,
    graph: Graph<V>,
    last_input: f32,
    last_output: Vec<(f32, f32)>,
    resolution: usize,
}

impl<V> Fuzzy<V>
where
    V: Eq + Hash + Copy + Display,
{
    pub fn new(
        functions: HashMap<V, Box<dyn Fn(f32) -> f32>>,
        range: Range<f32>,
        pos: (f32, f32),
        size: (f32, f32),
    ) -> Fuzzy<V> {
        let f = Rc::new(functions);
        Fuzzy {
            members: f.len(),
            range,
            graph: Graph::new(
                f.keys().map(|x| x.to_string()).collect(),
                pos,
                size,
                Rc::clone(&f),
                None,
            ),
            functions: f,
            last_input: 0.,
            resolution: 100,
            last_output: vec![(0., 0.); 101],
        }
    }
    pub fn fuzzify(&mut self, x: f32) -> HashMap<V, f32> {
        self.last_input = (x - self.range.start) / (self.range.end - self.range.start);
        let mut result = HashMap::with_capacity(self.members);
        for (&l, f) in self.functions.iter() {
            result.insert(l, f(self.last_input));
        }
        println!("{}", x);
        result
    }

    pub fn defuzzify(&mut self, acuts: HashMap<V, f32>) -> f32 {
        if acuts.len() != self.members {
            panic!(
                "Length of alpha cuts ({}) != Length of membership functions ({})",
                acuts.len(),
                self.members
            );
        }
        let (mut mx, mut my, mut m) = (0., 0., 0.);
        for i in 0..self.resolution {
            let x = i as f32 / (self.resolution - 1) as f32;
            let y = acuts
                .iter()
                .fold(0f32, |acc, (l, &a)| acc.max(self.functions[l](x).min(a)));
            self.last_output[i + 1] = (x, y);
            mx += y * x;
            my += y * y;
            m += y;
            // println!("{} {} {}", mx, my, m);
        }
        mx /= m;
        my /= 2. * m;
        self.last_output[0] = (mx, my);
        mx * (self.range.end - self.range.start) + self.range.start
    }

    pub fn draw(&self, ctx: &Context, is_output: bool) {
        self.graph.draw(
            ctx,
            if is_output {
                None
            } else {
                Some(self.last_input)
            },
            if is_output {
                Some(&self.last_output)
            } else {
                None
            },
        );
    }
}
