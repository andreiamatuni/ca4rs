use std::collections::HashMap;

use anyhow::Result;

pub struct Rule {
    pub func: HashMap<String, u8>,
}

impl Rule {
    /// Build a new rule from an array of output maps
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
    pub fn new(map: [u8; 8]) -> Self {
        let mut func = HashMap::new();
        func.insert("111".into(), map[0]);
        func.insert("110".into(), map[1]);
        func.insert("101".into(), map[2]);
        func.insert("100".into(), map[3]);
        func.insert("011".into(), map[4]);
        func.insert("010".into(), map[5]);
        func.insert("001".into(), map[6]);
        func.insert("000".into(), map[7]);
        Self { func }
    }

    pub fn lookup(self, input: String) -> u8 {
        self.func.get(&input).unwrap().to_owned()
    }
}
