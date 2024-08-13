use std::path::Path;

use anyhow::Result;
use ndarray::prelude::*;
use ndarray_npy::WriteNpyExt;
use ndarray_npy::{read_npy, write_npy};

// pub fn save<P>(ca: &Array2<u8>, path: P) -> Result<()>
// where
//     P: AsRef<Path>,
// {
//     write_npy(path, ca)?;
//     Ok(())
// }
