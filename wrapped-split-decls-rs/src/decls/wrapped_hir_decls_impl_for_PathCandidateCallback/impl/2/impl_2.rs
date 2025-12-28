use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < F > PathCandidateCallback for F where F : FnMut (AssocItem) -> ControlFlow < () > , { fn on_inherent_item (& mut self , item : AssocItem) -> ControlFlow < () > { self (item) } fn on_trait_item (& mut self , item : AssocItem) -> ControlFlow < () > { self (item) } }