use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Possible styles for suggestion subdiagnostics."] # [derive (Clone , Copy , PartialEq)] pub (super) enum SuggestionKind { Normal , Short , Hidden , Verbose , ToolOnly , }