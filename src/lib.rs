pub mod ca;
pub mod io;
pub mod rule;
pub mod vis;

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;

    #[test]
    pub fn rule30() -> Result<()> {
        let r = rule::Rule::new([0, 0, 0, 1, 1, 1, 1, 0])?;

        let input_size = 101;
        let input = ca::new_default_input(input_size);

        let ca = ca::CA::new(r, input);

        let output = ca.simulate(input_size);
        vis::draw("output/output.png", &output)?;
        Ok(())
    }
}
