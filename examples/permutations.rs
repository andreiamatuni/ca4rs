use ca4rs::{ca, vis, Rule, CA};

fn main() {
    // define the Rule
    let r = Rule::number(30).unwrap();

    // simulate CA's with the given rule for all possible permutations
    // of input size of 10 for 100 steps
    let results = ca::simulate_permutations(r, 10, 100, None);
}
