// Generated macro for impl_344 (impl)
macro_rules! Depcrate_config_optionsimpl_344 {
() => {
// Module: crate::config::options
// Provides: {"impl_344"}
// Dependencies: {}
impl Serialize for IgnoreList { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut seq = serializer . serialize_seq (Some (self . path_set . len ())) ? ; for e in & self . path_set { seq . serialize_element (e) ? ; } seq . end () } }
};
}
