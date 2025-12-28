use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct StringWrapper < 'a > (& 'a str) ;
}