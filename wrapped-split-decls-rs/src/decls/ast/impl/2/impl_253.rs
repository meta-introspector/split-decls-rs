use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ForeignItemKind { pub fn ident (& self) -> Option < Ident > { match * self { ForeignItemKind :: Static (box StaticItem { ident , .. }) | ForeignItemKind :: Fn (box Fn { ident , .. }) | ForeignItemKind :: TyAlias (box TyAlias { ident , .. }) => Some (ident) , ForeignItemKind :: MacCall (_) => None , } } }
}