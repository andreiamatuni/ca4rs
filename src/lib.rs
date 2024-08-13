pub mod ca;
pub mod io;
pub mod rule;
pub mod vis;

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;
    use ndarray::prelude::*;
    use rule::Rule;

    #[test]
    pub fn basic() -> Result<()> {
        let input_size = 301;

        let r = Rule::number(30)?;
        let input = ca::new_default_input(input_size);
        let ca = ca::CA::new(r, input);
        let output = ca.simulate(input_size);
        let truncated_out = output.slice_move(s![0..input_size / 2, 0..]);
        vis::draw("output/rule_30.png", &truncated_out)?;

        let r = Rule::number(110)?;
        let input = ca::new_default_input(input_size);
        let ca = ca::CA::new(r, input);
        let output = ca.simulate(input_size);
        let truncated_out = output.slice_move(s![0..input_size / 2, 0..]);
        vis::draw("output/rule_110.png", &truncated_out)?;

        let r = Rule::number(122)?;
        let input = ca::new_default_input(input_size);
        let ca = ca::CA::new(r, input);
        let output = ca.simulate(input_size);
        let truncated_out = output.slice_move(s![0..input_size / 2, 0..]);
        vis::draw("output/rule_122.png", &truncated_out)?;

        let r = Rule::number(126)?;
        let input = ca::new_default_input(input_size);
        let ca = ca::CA::new(r, input);
        let output = ca.simulate(input_size);
        let truncated_out = output.slice_move(s![0..input_size / 2, 0..]);
        vis::draw("output/rule_126.png", &truncated_out)?;
        Ok(())
    }
}
