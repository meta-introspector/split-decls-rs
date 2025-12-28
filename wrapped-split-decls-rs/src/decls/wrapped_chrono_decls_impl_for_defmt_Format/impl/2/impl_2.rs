use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (feature = "defmt")] impl defmt :: Format for OutOfRange { fn format (& self , fmt : defmt :: Formatter) { defmt :: write ! (fmt , "out of range") ; } }
}