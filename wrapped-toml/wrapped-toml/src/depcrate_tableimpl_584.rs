// Generated macro for impl_584 (impl)
macro_rules! Depcrate_tableimpl_584 {
() => {
// Module: crate::table
// Provides: {"impl_584"}
// Dependencies: {}
impl ser :: Serialize for Table { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { use serde_core :: ser :: SerializeMap ; let mut map = serializer . serialize_map (Some (self . len ())) ? ; for (k , v) in self { map . serialize_key (k) ? ; map . serialize_value (v) ? ; } map . end () } }
};
}
