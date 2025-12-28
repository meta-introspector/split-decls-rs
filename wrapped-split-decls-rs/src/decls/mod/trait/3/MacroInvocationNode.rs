use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait MacroInvocationNode : HasAttrs + HasNodeId + Sized { type ItemKind ; fn is_mac_call (& self) -> bool ; fn take_mac_call (self) -> (ast :: MacCall , AttrVec , AddSemicolon) ; fn delegation (& self) -> Option < (& ast :: MacCall , & ast :: AssocItem) > ; fn delegation_item_kind (_deleg : Box < ast :: Delegation >) -> Self :: ItemKind ; fn from_item (_item : ast :: Item < Self :: ItemKind >) -> Self ; }
}