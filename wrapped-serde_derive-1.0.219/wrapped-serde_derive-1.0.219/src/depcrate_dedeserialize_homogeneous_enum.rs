// Generated macro for deserialize_homogeneous_enum (function)
macro_rules! Depcrate_dedeserialize_homogeneous_enum {
() => {
// Module: crate::de
// Provides: {"deserialize_homogeneous_enum"}
// Dependencies: {}
fn deserialize_homogeneous_enum (params : & Parameters , variants : & [Variant] , cattrs : & attr :: Container ,) -> Fragment { match cattrs . tag () { attr :: TagType :: External => deserialize_externally_tagged_enum (params , variants , cattrs) , attr :: TagType :: Internal { tag } => { deserialize_internally_tagged_enum (params , variants , cattrs , tag) } attr :: TagType :: Adjacent { tag , content } => { deserialize_adjacently_tagged_enum (params , variants , cattrs , tag , content) } attr :: TagType :: None => deserialize_untagged_enum (params , variants , cattrs) , } }
};
}
