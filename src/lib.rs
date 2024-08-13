pub mod ca;
pub mod io;
pub mod rule;
pub mod vis;

pub use ca::CA;
pub use rule::Rule;

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
        let mut ca = ca::CA::new(r, input);
        ca.simulate(input_size / 2);
        vis::draw("output/rule_30.png", &ca.output.as_ref().unwrap())?;

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
}
