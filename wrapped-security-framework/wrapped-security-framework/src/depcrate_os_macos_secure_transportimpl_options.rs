// Generated macro for impl_options (macro)
macro_rules! Depcrate_os_macos_secure_transportimpl_options {
() => {
// Module: crate::os::macos::secure_transport
// Provides: {"impl_options"}
// Dependencies: {}
macro_rules ! impl_options { ($ ($ (# [$ a : meta]) * const $ opt : ident : $ get : ident & $ set : ident ,) *) => { $ (# [allow (deprecated)] $ (# [$ a]) * # [inline] fn $ set (& mut self , value : bool) -> Result < () > { unsafe { cvt (SSLSetSessionOption (self . as_inner () , $ opt , :: core_foundation :: base :: Boolean :: from (value))) } } # [allow (deprecated)] $ (# [$ a]) * # [inline] fn $ get (& self) -> Result < bool > { let mut value = 0 ; unsafe { cvt (SSLGetSessionOption (self . as_inner () , $ opt , & mut value)) ?; } Ok (value != 0) }) * } }
};
}
