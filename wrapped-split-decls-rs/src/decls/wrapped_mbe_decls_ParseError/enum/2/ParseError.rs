use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , PartialEq , Eq , Clone)] pub enum ParseError { UnexpectedToken (Box < str >) , Expected (Box < str >) , InvalidRepeat , RepetitionEmptyTokenTree , }
}