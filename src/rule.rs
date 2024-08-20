
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Rule {
    pub f1: u8,
    pub f2: u8,
    pub f3: u8,
    pub f4: u8,
    pub f5: u8,
    pub f6: u8,
    pub f7: u8,
    pub f8: u8,
}

impl Rule {
    /// Build a new rule from an array of output mappings
    ///
    /// Elements in the array determine the output rules for
    /// neighborhoods with the following order:
    ///
    ///     - 111
    ///     - 110
    ///     - 101
    ///     - 100
    ///     - 011
    ///     - 010
    ///     - 001
    ///     - 000
    ///
    ///  The array must be of length 8. So given an array such as
    ///  [0, 1, 1, 0, 1, 1, 1, 0], the neighborhood of 111 would
    ///  produce output 0, neighborhood 110 would produce output 1,
    ///  neighborhood of 101 would produce output 1, neighborhood of
    ///  100 would produce output 0, etc...
    pub fn new(map: [u8; 8]) -> Result<Self> {
        for x in map {
            if ![0, 1].contains(&x) {
                return Err(anyhow::format_err!(
                    "invalid rule (values must be either 0 or 1) found: {:?}",
                    x
                ));
            }
        }

        Ok(Self {
            f1: map[0],
            f2: map[1],
            f3: map[2],
            f4: map[3],
            f5: map[4],
            f6: map[5],
            f7: map[6],
            f8: map[7],
        })
    }

    /// Create a Rule based on the standard set of 256 rule numberings
    pub fn number(num: u8) -> Result<Self> {
        let str_rep = format!("{num:08b}");
        let array: Vec<u8> = str_rep
            .chars()
            .map(|x| x.to_digit(2).unwrap() as u8)
            .collect();

        Rule::new(array.try_into().unwrap())
    }

    /// Lookup the output given a neighborhood
    pub(crate) fn lookup(&self, input: u8) -> u8 {
        match input {
            7 => self.f1,
            6 => self.f2,
            5 => self.f3,
            4 => self.f4,
            3 => self.f5,
            2 => self.f6,
            1 => self.f7,
            0 => self.f8,
            _ => 0,
        }
    }
}
