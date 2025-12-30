// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl Serialize for SerializeFieldSet < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut seq = serializer . serialize_seq (Some (self . 0 . len ())) ? ; for element in self . 0 { seq . serialize_element (& SerializeField (& element)) ? ; } seq . end () } }
};
}
