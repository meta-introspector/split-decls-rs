use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl From < ForeignItemKind > for ItemKind { fn from (foreign_item_kind : ForeignItemKind) -> ItemKind { match foreign_item_kind { ForeignItemKind :: Static (box static_foreign_item) => { ItemKind :: Static (Box :: new (static_foreign_item)) } ForeignItemKind :: Fn (fn_kind) => ItemKind :: Fn (fn_kind) , ForeignItemKind :: TyAlias (ty_alias_kind) => ItemKind :: TyAlias (ty_alias_kind) , ForeignItemKind :: MacCall (a) => ItemKind :: MacCall (a) , } } }