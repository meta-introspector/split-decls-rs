use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Whether a `Punct` is followed immediately by another `Punct` or followed by"] # [doc = " another token or whitespace."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum Spacing { # [doc = " E.g. `+` is `Alone` in `+ =`, `+ident` or `+()`."] Alone , # [doc = " E.g. `+` is `Joint` in `+=` or `'` is `Joint` in `'#`."] # [doc = ""] # [doc = " Additionally, single quote `'` can join with identifiers to form"] # [doc = " lifetimes `'ident`."] Joint , }
}