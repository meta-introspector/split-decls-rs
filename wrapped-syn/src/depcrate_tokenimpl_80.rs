// Generated macro for impl_80 (impl)
macro_rules! Depcrate_tokenimpl_80 {
() => {
// Module: crate::token
// Provides: {"impl_80"}
// Dependencies: {}
impl Group { # [cfg (feature = "printing")] # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] pub fn surround < F > (& self , tokens : & mut TokenStream , f : F) where F : FnOnce (& mut TokenStream) , { let mut inner = TokenStream :: new () ; f (& mut inner) ; printing :: delim (Delimiter :: None , self . span , tokens , inner) ; } }
};
}
