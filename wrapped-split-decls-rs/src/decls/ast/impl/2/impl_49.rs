use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl GenericBound { pub fn span (& self) -> Span { match self { GenericBound :: Trait (t , ..) => t . span , GenericBound :: Outlives (l) => l . ident . span , GenericBound :: Use (_ , span) => * span , } } }
}