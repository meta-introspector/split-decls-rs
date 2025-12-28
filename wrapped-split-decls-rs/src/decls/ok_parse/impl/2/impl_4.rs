use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > ParseResultBase < T > for OkParse < T > { fn handle_failure (& mut self , _tracker : & mut DynMTrackerTrait ! ()) { } fn is_ok (& self) -> bool { true } fn unwrap (self) -> Option < T > { Some (self . 0) } }
}