use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FromStr for SuggestionKind { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "normal" => Ok (SuggestionKind :: Normal) , "short" => Ok (SuggestionKind :: Short) , "hidden" => Ok (SuggestionKind :: Hidden) , "verbose" => Ok (SuggestionKind :: Verbose) , "tool-only" => Ok (SuggestionKind :: ToolOnly) , _ => Err (()) , } } }
}