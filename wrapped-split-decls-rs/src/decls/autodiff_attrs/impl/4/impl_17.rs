use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FromStr for DiffActivity { type Err = () ; fn from_str (s : & str) -> Result < DiffActivity , () > { match s { "None" => Ok (DiffActivity :: None) , "Active" => Ok (DiffActivity :: Active) , "ActiveOnly" => Ok (DiffActivity :: ActiveOnly) , "Const" => Ok (DiffActivity :: Const) , "Dual" => Ok (DiffActivity :: Dual) , "Dualv" => Ok (DiffActivity :: Dualv) , "DualOnly" => Ok (DiffActivity :: DualOnly) , "DualvOnly" => Ok (DiffActivity :: DualvOnly) , "Duplicated" => Ok (DiffActivity :: Duplicated) , "DuplicatedOnly" => Ok (DiffActivity :: DuplicatedOnly) , _ => Err (()) , } } }
}