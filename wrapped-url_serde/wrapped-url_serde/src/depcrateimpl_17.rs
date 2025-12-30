// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
# [doc = " Serializes this Option<URL> into a `serde` stream."] impl < 'a > Serialize for Ser < 'a , Option < Url > > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer { if let Some (url) = self . 0 . as_ref () { serializer . serialize_some (url . as_str ()) } else { serializer . serialize_none () } } }
};
}
