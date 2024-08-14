pub use ca4rs::ca::CA;
pub use ca4rs::rule::Rule;

#[cfg(test)]
mod tests {

    use anyhow::Result;
    use ca4rs::rule::Rule;
    use ca4rs::*;
    use ndarray::prelude::*;

    #[test]
    pub fn basic() -> Result<()> {
        let input_size = 301;

        let r = Rule::number(30)?;
        let input = ca::new_default_input(input_size);
        let mut ca = ca::CA::new(r, input);
        ca.simulate(input_size / 2);
        vis::draw(
            "output/rule_30.png",
            &ca.output.as_ref().unwrap(),
            8096,
            4048,
        )?;

        Ok(())
    }

    #[test]
    pub fn generate_permuations() -> Result<()> {
        let perms = ca::permutations(3);
        assert_eq!(perms.len(), 8);

        println!("{:?}", perms);
        Ok(())
    }

    #[test]
    pub fn simulate_permutations() -> Result<()> {
        let results = ca::simulate_permutations(Rule::number(30).unwrap(), 3, 100, None);

        assert_eq!(results.len(), 8);
        Ok(())
    }

    #[test]
    pub fn random_input() -> Result<()> {
        let input_size = 501;
        let rule_number = 122;

        let r = Rule::number(rule_number)?;
        let input = ca::random_input(input_size);
        let mut ca = ca::CA::new(r, input);
        ca.simulate(input_size);
        vis::draw(
            &format!("output/rule_{rule_number}_random_input.png"),
            &ca.output.as_ref().unwrap(),
            8096,
            8096,
        )?;

        Ok(())
    }
}
