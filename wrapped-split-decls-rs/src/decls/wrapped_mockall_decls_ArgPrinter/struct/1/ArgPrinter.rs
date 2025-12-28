use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc (hidden)] pub struct ArgPrinter < 'a , T > (pub & 'a T) ;
}