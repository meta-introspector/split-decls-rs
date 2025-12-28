use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " The kind of macro invocation or definition."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Encodable , Decodable , Hash , Debug)] # [derive (HashStable_Generic)] pub enum MacroKind { # [doc = " A bang macro `foo!()`."] Bang , # [doc = " An attribute macro `#[foo]`."] Attr , # [doc = " A derive macro `#[derive(Foo)]`"] Derive , }
}