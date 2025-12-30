// Generated macro for impl_188 (impl)
macro_rules! Depcrateimpl_188 {
() => {
// Module: crate
// Provides: {"impl_188"}
// Dependencies: {}
# [doc = " Serializes this URL into a `serde` stream."] # [doc = ""] # [doc = " This implementation is only available if the `serde` Cargo feature is enabled."] # [cfg (feature = "serde")] impl serde :: Serialize for Url { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . serialize_str (self . as_str ()) } }
};
}
