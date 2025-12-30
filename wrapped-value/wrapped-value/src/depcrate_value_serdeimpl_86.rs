// Generated macro for impl_86 (impl)
macro_rules! Depcrate_value_serdeimpl_86 {
() => {
// Module: crate::value_serde
// Provides: {"impl_86"}
// Dependencies: {}
impl Serialize for SerdeVariable { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { let mut s = serializer . serialize_map (Some (1)) ? ; s . serialize_entry ("$var" , & self . 0) ? ; s . end () } }
};
}
