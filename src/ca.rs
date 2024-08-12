use anyhow::Result;

use crate::rule;

/// Cellular Automata
///
/// This CA implements a given rule in a particular dimension
pub struct CA {
    pub size: usize,
    pub rule: rule::Rule,
    pub input: Vec<u8>,
}

impl CA {
    pub fn new(size: usize, rule: rule::Rule, input: Vec<u8>) -> Self {
        Self { size, rule, input }
    }

    pub fn simulate(self, len: usize) -> Vec<Vec<u8>> {
        let mut result: Vec<Vec<u8>> = vec![];
        result.push(self.input.clone());

        let mut curr_row = self.input.clone();
        let mut next_row = curr_row.clone();

        for _ in 0..len {
            for i in 1..curr_row.len() - 1 {
                let neighbors = self.cells_to_str(i, &curr_row);
                let cell_output = self.rule.func.get(&neighbors).unwrap().to_owned();

                next_row[i] = cell_output;
            }

            curr_row = next_row.clone();
            result.push(next_row.clone());
        }
        result
    }

    fn cells_to_str(&self, idx: usize, row: &Vec<u8>) -> String {
        let string = format!("{}{}{}", row[idx - 1], row[idx], row[idx + 1]);
        string
    }
}
