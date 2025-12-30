// Generated macro for impl_91 (impl)
macro_rules! Depcrate_cldr_serde_caimpl_91 {
() => {
// Module: crate::cldr_serde::ca
// Provides: {"impl_91"}
// Dependencies: {}
impl Eras { # [doc = " Load the era corresponding to a [`Length`] value"] # [doc = ""] # [doc = " Panics on Length::Short"] pub (crate) fn load (& self , length : Length) -> & HashMap < String , String > { match length { Length :: Abbr => & self . abbr , Length :: Narrow => & self . narrow , Length :: Wide => & self . names , Length :: Short | Length :: Numeric => { unreachable ! ("Years do not have short/numeric symbols!") } } } }
};
}
