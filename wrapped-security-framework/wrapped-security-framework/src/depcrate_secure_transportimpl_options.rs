// Generated macro for impl_options (macro)
macro_rules! Depcrate_secure_transportimpl_options {
() => {
// Module: crate::secure_transport
// Provides: {"impl_options"}
// Dependencies: {}
macro_rules ! impl_options { ($ ($ (# [$ a : meta]) * const $ opt : ident : $ get : ident & $ set : ident ,) *) => { $ (# [allow (deprecated)] $ (# [$ a]) * # [inline (always)] pub fn $ set (& mut self , value : bool) -> Result < () > { unsafe { cvt (SSLSetSessionOption (self . 0 , $ opt , Boolean :: from (value))) } } # [allow (deprecated)] $ (# [$ a]) * # [inline] pub fn $ get (& self) -> Result < bool > { let mut value = 0 ; unsafe { cvt (SSLGetSessionOption (self . 0 , $ opt , & mut value)) ?; } Ok (value != 0) }) * } }
};
}
