use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Represents the help messages seen on a diagnostic."] # [derive (Clone , Debug , PartialEq , Hash , Encodable , Decodable)] pub enum Suggestions { # [doc = " Indicates that new suggestions can be added or removed from this diagnostic."] # [doc = ""] # [doc = " `DiagInner`'s new_* methods initialize the `suggestions` field with"] # [doc = " this variant. Also, this is the default variant for `Suggestions`."] Enabled (Vec < CodeSuggestion >) , # [doc = " Indicates that suggestions cannot be added or removed from this diagnostic."] # [doc = ""] # [doc = " Gets toggled when `.seal_suggestions()` is called on the `DiagInner`."] Sealed (Box < [CodeSuggestion] >) , # [doc = " Indicates that no suggestion is available for this diagnostic."] # [doc = ""] # [doc = " Gets toggled when `.disable_suggestions()` is called on the `DiagInner`."] Disabled , }
}