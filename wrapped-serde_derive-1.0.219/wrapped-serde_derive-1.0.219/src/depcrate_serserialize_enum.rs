// Generated macro for serialize_enum (function)
macro_rules! Depcrate_serserialize_enum {
() => {
// Module: crate::ser
// Provides: {"serialize_enum"}
// Dependencies: {}
fn serialize_enum (params : & Parameters , variants : & [Variant] , cattrs : & attr :: Container) -> Fragment { assert ! (variants . len () as u64 <= u64 :: from (u32 :: MAX)) ; let self_var = & params . self_var ; let mut arms : Vec < _ > = variants . iter () . enumerate () . map (| (variant_index , variant) | { serialize_variant (params , variant , variant_index as u32 , cattrs) }) . collect () ; if cattrs . remote () . is_some () && cattrs . non_exhaustive () { arms . push (quote ! { ref unrecognized => _serde :: __private :: Err (_serde :: ser :: Error :: custom (_serde :: __private :: ser :: CannotSerializeVariant (unrecognized))) , }) ; } quote_expr ! { match *# self_var { # (# arms) * } } }
};
}
