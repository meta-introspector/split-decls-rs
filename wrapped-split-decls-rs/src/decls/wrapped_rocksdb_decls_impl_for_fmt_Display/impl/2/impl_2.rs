use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Display for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { self . message . fmt (formatter) } }
}