use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FromStr for Applicability { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "machine-applicable" => Ok (Applicability :: MachineApplicable) , "maybe-incorrect" => Ok (Applicability :: MaybeIncorrect) , "has-placeholders" => Ok (Applicability :: HasPlaceholders) , "unspecified" => Ok (Applicability :: Unspecified) , _ => Err (()) , } } }
}