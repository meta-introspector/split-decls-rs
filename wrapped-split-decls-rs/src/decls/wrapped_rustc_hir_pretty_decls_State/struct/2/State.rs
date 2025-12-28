use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct State < 'a > { pub s : pp :: Printer , comments : Option < Comments < 'a > > , attrs : & 'a dyn Fn (HirId) -> & 'a [hir :: Attribute] , ann : & 'a (dyn PpAnn + 'a) , }
}