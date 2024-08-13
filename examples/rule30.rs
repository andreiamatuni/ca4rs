use ca4rs::{ca, vis, Rule, CA};

fn main() {
    let input_size = 301;

    // define the Rule
    let r = Rule::number(30).unwrap();
    // define the initial input
    let input = ca::new_default_input(input_size);
    // build the cellular automata
    let mut ca = CA::new(r, input);
    // simulate some number of steps (here it's N = half the input length)
    let output = ca.simulate(input_size / 2);
    // plot the simulation
    vis::draw("rule_30.png", &output).unwrap();
}
