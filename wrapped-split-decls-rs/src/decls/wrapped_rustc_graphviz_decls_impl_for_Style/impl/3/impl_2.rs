use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Style { pub fn as_slice (self) -> & 'static str { match self { Style :: None => "" , Style :: Solid => "solid" , Style :: Dashed => "dashed" , Style :: Dotted => "dotted" , Style :: Bold => "bold" , Style :: Rounded => "rounded" , Style :: Diagonals => "diagonals" , Style :: Filled => "filled" , Style :: Striped => "striped" , Style :: Wedged => "wedged" , } } }