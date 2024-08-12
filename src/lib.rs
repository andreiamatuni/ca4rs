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
        let r = rule::Rule::number(30)?;

        let input_size = 101;
        let input = ca::new_default_input(input_size);
        let ca = ca::CA::new(r, input);
        let output = ca.simulate(input_size);
        vis::draw("output/rule_30.png", &output)?;

        let r = rule::Rule::number(110)?;
        let input_size = 101;
        let input = ca::new_default_input(input_size);
        let ca = ca::CA::new(r, input);
        let output = ca.simulate(input_size);
        vis::draw("output/rule_110.png", &output)?;

        let r = rule::Rule::number(122)?;
        let input_size = 101;
        let input = ca::new_default_input(input_size);
        let ca = ca::CA::new(r, input);
        let output = ca.simulate(input_size);
        vis::draw("output/rule_122.png", &output)?;

        let r = rule::Rule::number(126)?;
        let input_size = 101;
        let input = ca::new_default_input(input_size);
        let ca = ca::CA::new(r, input);
        let output = ca.simulate(input_size);
        vis::draw("output/rule_126.png", &output)?;
        Ok(())
    }
}
