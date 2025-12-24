use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl FromStr for Bom {
    /// A `std::io::Error` instance returned by `std::fs::File::open`.
    type Err = Error;
    /// Parse the BOM type from the file located at `path`.
    fn from_str(path: &str) -> Result<Self, Self::Err> {
        let mut file = File::open(path)?;
        Ok(Bom::from(&mut file))
    }
}
