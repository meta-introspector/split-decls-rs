// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
# [doc = " Serializes this URL into a `serde` stream."] impl < 'a > Serialize for Ser < 'a , Url > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer { serializer . serialize_str (self . 0 . as_str ()) } }
};
}
