use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FromStr for DiffMode { type Err = () ; fn from_str (s : & str) -> Result < DiffMode , () > { match s { "Error" => Ok (DiffMode :: Error) , "Source" => Ok (DiffMode :: Source) , "Forward" => Ok (DiffMode :: Forward) , "Reverse" => Ok (DiffMode :: Reverse) , _ => Err (()) , } } }
}