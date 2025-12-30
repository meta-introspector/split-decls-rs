// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl Serialize for SerializeId < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_tuple_struct ("Id" , 1) ? ; state . serialize_field (& self . 0 . into_u64 ()) ? ; state . end () } }
};
}
