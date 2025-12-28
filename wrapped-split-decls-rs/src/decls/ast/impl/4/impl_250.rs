use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < AssocItemKind > for ItemKind { fn from (assoc_item_kind : AssocItemKind) -> ItemKind { match assoc_item_kind { AssocItemKind :: Const (item) => ItemKind :: Const (item) , AssocItemKind :: Fn (fn_kind) => ItemKind :: Fn (fn_kind) , AssocItemKind :: Type (ty_alias_kind) => ItemKind :: TyAlias (ty_alias_kind) , AssocItemKind :: MacCall (a) => ItemKind :: MacCall (a) , AssocItemKind :: Delegation (delegation) => ItemKind :: Delegation (delegation) , AssocItemKind :: DelegationMac (delegation) => ItemKind :: DelegationMac (delegation) , } } }
}