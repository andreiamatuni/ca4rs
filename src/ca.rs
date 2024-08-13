use anyhow::Result;
use ndarray::prelude::*;

use crate::rule;

/// Cellular Automata
///
/// This CA implements a given rule with an initial input state
pub struct CA {
    pub rule: rule::Rule,
    pub input: Vec<u8>,
}

impl CA {
    /// Build a new CA given a [Rule] and an initial input
    pub fn new(rule: rule::Rule, input: Vec<u8>) -> Self {
        Self { rule, input }
    }

    pub fn simulate(self, len: usize) -> Array2<u8> {
        let mut res = ndarray::Array2::<u8>::zeros((len, self.input.len()));
        let input = Array1::<u8>::from_vec(self.input.clone());
        res.slice_mut(s![0, 0..]).assign(&input);

        for sim in 0..len - 1 {
            for i in 1..res.shape()[1] - 1 {
                let neighbors = self.cells_to_str(i, &res.slice(s![sim, 0..]));
                let cell_output = self.rule.lookup(&neighbors);
                res[[sim + 1, i]] = cell_output;
            }
        }
        res
    }

    fn cells_to_str(&self, idx: usize, row: &ArrayView1<u8>) -> String {
        let string = format!("{}{}{}", row[idx - 1], row[idx], row[idx + 1]);
        string
    }
}

/// Generate a default input with a single "on" cell in the center
pub fn new_default_input(size: usize) -> Vec<u8> {
    let mut input: Vec<u8> = vec![0; size];
    let middle = size / 2;
    input[middle] = 1;
    input
}
