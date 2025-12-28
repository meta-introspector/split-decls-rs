use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 't > RegexCaptures for DummyRegexCaptures { fn get (& self , _i : usize) -> Option < & str > { None } fn len (& self) -> usize { 0 } }
}