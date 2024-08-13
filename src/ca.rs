use anyhow::Result;
use itertools::Itertools;
use ndarray::prelude::*;

use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
};

use crate::rule;

/// Cellular Automata
///
/// This CA implements a given rule with an initial input state
#[derive(Debug)]
pub struct CA {
    pub rule: rule::Rule,
    input: Vec<u8>,
    pub output: Option<Array2<u8>>,
}

impl CA {
    /// Build a new CA given a [Rule] and an initial input
    pub fn new(rule: rule::Rule, input: Vec<u8>) -> Self {
        Self {
            rule,
            input,
            output: None,
        }
    }

    /// Compute the outputs of the CA
    pub fn simulate(&mut self, len: usize) {
        // build the output matrix
        let mut res = ndarray::Array2::<u8>::zeros((len, self.input.len()));

        // assign the input to the first row of the output
        let input = Array1::<u8>::from_vec(self.input.clone());
        res.slice_mut(s![0, 0..]).assign(&input);

        for sim in 0..len - 1 {
            for i in 1..res.shape()[1] - 1 {
                let neighbors = self.cells_to_str(i, &res.slice(s![sim, 0..]));
                let cell_output = self.rule.lookup(&neighbors);
                res[[sim + 1, i]] = cell_output;
            }
        }

        self.output = Some(res.clone());
    }

    /// Return the initial state of the CA
    pub fn input(&self) -> Vec<u8> {
        self.input.clone()
    }

    /// Count the number of cells in an "on" state
    pub fn num_cells_on(&self) -> Result<usize> {
        match self.output {
            Some(ref s) => Ok(s.clone().mapv_into_any(|x| x as usize).sum()),
            None => Err(anyhow::format_err!("need to compute simulation first")),
        }
    }

    /// Count number of cells in an "off" state
    pub fn num_cells_off(&self) -> Result<usize> {
        let num_on = self.num_cells_on()?;
        let shape = self.output.as_ref().unwrap().shape();
        let total_count = shape[0] * shape[1];
        Ok(total_count - num_on)
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

/// Generate all the unique permutations of input for a given size
pub fn permutations(input_size: usize) -> Vec<Vec<u8>> {
    itertools::repeat_n(0..=1, input_size)
        .multi_cartesian_product()
        .collect_vec()
}

/// Simulate a CA with a given rule for all possible initial input states.
/// The function will default to using the number of threads equal to the
/// available parallelism on the target machine (according to
/// std::thread::available_parallelism). This can be overridden with the
/// num_threads parameter. The memory complexity of this function scales
/// exponentially with the input size.
pub fn simulate_permutations(
    rule: rule::Rule,
    input_size: usize,
    sim_length: usize,
    num_threads: Option<usize>,
) -> Vec<CA> {
    let results: Arc<Mutex<Vec<CA>>> = Arc::new(Mutex::new(vec![]));
    let perms = permutations(input_size);
    println!("num perms: {}", perms.len());

    let threads;
    match num_threads {
        Some(n) => {
            threads = n;
        }
        None => {
            threads = thread::available_parallelism().unwrap().get();
        }
    }

    let chunk_size = match perms.len() / threads {
        0 => 1,
        _ => perms.len() / threads,
    };

    let input_chunks: Vec<Vec<Vec<u8>>> = perms
        .into_iter()
        .chunks(chunk_size)
        .into_iter()
        .map(|c| c.collect_vec())
        .collect();

    let mut handles = vec![];
    for chunk in input_chunks.into_iter() {
        let rule = rule.clone();
        let results = results.clone();
        let h = thread::spawn(move || {
            for input in chunk {
                let mut ca = CA::new(rule.clone(), input);
                ca.simulate(sim_length);
                results.lock().unwrap().push(ca);
            }
        });
        handles.push(h);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let inner = Arc::into_inner(results).unwrap();
    Mutex::into_inner(inner).unwrap()
}
