use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str ("Block buffer error") } }
}