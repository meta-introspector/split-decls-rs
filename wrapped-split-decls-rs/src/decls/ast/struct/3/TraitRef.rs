use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " `TraitRef`s appear in impls."] # [doc = ""] # [doc = " Resolution maps each `TraitRef`'s `ref_id` to its defining trait; that's all"] # [doc = " that the `ref_id` is for. The `impl_id` maps to the \"self type\" of this impl."] # [doc = " If this impl is an `ItemKind::Impl`, the `impl_id` is redundant (it could be the"] # [doc = " same as the impl's `NodeId`)."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct TraitRef { pub path : Path , pub ref_id : NodeId , }
}