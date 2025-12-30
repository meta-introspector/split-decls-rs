// Generated macro for deserialize_enum (function)
macro_rules! Depcrate_dedeserialize_enum {
() => {
// Module: crate::de
// Provides: {"deserialize_enum"}
// Dependencies: {}
fn deserialize_enum (params : & Parameters , variants : & [Variant] , cattrs : & attr :: Container ,) -> Fragment { match variants . iter () . position (| var | var . attrs . untagged ()) { Some (variant_idx) => { let (tagged , untagged) = variants . split_at (variant_idx) ; let tagged_frag = Expr (deserialize_homogeneous_enum (params , tagged , cattrs)) ; deserialize_untagged_enum_after (params , untagged , cattrs , Some (tagged_frag)) } None => deserialize_homogeneous_enum (params , variants , cattrs) , } }
};
}
