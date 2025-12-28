use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait PathCandidateCallback { fn on_inherent_item (& mut self , item : AssocItem) -> ControlFlow < () > ; fn on_trait_item (& mut self , item : AssocItem) -> ControlFlow < () > ; }