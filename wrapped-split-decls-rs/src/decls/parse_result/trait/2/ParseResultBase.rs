use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait ParseResultBase < T > { fn handle_failure (& mut self , tracker : & mut DynMTrackerTrait ! ()) ; fn is_ok (& self) -> bool ; fn unwrap (self) -> Option < T > ; }