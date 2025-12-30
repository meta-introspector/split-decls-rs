// Generated macro for impl_256 (impl)
macro_rules! Depcrate_de_implsimpl_256 {
() => {
// Module: crate::de::impls
// Provides: {"impl_256"}
// Dependencies: {}
# [cfg (all (feature = "std" , any (unix , windows)))] impl < 'de > Visitor < 'de > for OsStringVisitor { type Value = OsString ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("os string") } # [cfg (unix)] fn visit_enum < A > (self , data : A) -> Result < Self :: Value , A :: Error > where A : EnumAccess < 'de > , { use std :: os :: unix :: ffi :: OsStringExt ; match tri ! (data . variant ()) { (OsStringKind :: Unix , v) => v . newtype_variant () . map (OsString :: from_vec) , (OsStringKind :: Windows , _) => Err (Error :: custom ("cannot deserialize Windows OS string on Unix" ,)) , } } # [cfg (windows)] fn visit_enum < A > (self , data : A) -> Result < Self :: Value , A :: Error > where A : EnumAccess < 'de > , { use std :: os :: windows :: ffi :: OsStringExt ; match tri ! (data . variant ()) { (OsStringKind :: Windows , v) => v . newtype_variant :: < Vec < u16 > > () . map (| vec | OsString :: from_wide (& vec)) , (OsStringKind :: Unix , _) => Err (Error :: custom ("cannot deserialize Unix OS string on Windows" ,)) , } } }
};
}
