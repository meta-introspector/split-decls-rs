// Generated macro for impl_419 (impl)
macro_rules! Depcrate_ser_implsimpl_419 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_419"}
// Dependencies: {}
# [cfg (all (feature = "std" , any (unix , windows)))] # [cfg_attr (docsrs , doc (cfg (all (feature = "std" , any (unix , windows)))))] impl Serialize for OsStr { # [cfg (unix)] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { use std :: os :: unix :: ffi :: OsStrExt ; serializer . serialize_newtype_variant ("OsString" , 0 , "Unix" , self . as_bytes ()) } # [cfg (windows)] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { use std :: os :: windows :: ffi :: OsStrExt ; let val = self . encode_wide () . collect :: < Vec < _ > > () ; serializer . serialize_newtype_variant ("OsString" , 1 , "Windows" , & val) } }
};
}
