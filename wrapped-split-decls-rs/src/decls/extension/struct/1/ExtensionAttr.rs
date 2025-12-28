use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct ExtensionAttr { vis : Visibility , trait_ : Path , }
}