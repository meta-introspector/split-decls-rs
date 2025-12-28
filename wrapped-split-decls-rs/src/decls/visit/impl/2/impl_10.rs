use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl BoundKind { pub fn descr (self) -> & 'static str { match self { BoundKind :: Bound => "bounds" , BoundKind :: Impl => "`impl Trait`" , BoundKind :: TraitObject => "`dyn` trait object bounds" , BoundKind :: SuperTraits => "supertrait bounds" , } } }