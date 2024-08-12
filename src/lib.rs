pub mod ca;
pub mod io;
pub mod rule;
pub mod simulate;
pub mod vis;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use anyhow::Result;

    #[test]
    pub fn rule30() -> Result<()> {
        let r = rule::Rule::new([0, 0, 0, 1, 1, 1, 1, 0]);

        let input_size = 101;
        let mut input: Vec<u8> = vec![0; input_size];
        let middle = input_size / 2;
        input[middle] = 1;

        let ca = ca::CA::new(8, r, input);

        let output = ca.simulate(input_size);
        vis::draw("output.png", &output)?;
        Ok(())
    }
}
