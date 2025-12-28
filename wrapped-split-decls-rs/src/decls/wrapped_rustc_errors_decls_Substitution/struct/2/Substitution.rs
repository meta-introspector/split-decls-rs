use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Debug , PartialEq , Hash , Encodable , Decodable)] # [doc = " See the docs on `CodeSuggestion::substitutions`"] pub struct Substitution { pub parts : Vec < SubstitutionPart > , }