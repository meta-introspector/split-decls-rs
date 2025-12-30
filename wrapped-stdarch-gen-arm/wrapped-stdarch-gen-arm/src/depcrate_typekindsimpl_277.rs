// Generated macro for impl_277 (impl)
macro_rules! Depcrate_typekindsimpl_277 {
() => {
// Module: crate::typekinds
// Provides: {"impl_277"}
// Dependencies: {}
impl ToTokens for TypeKind { fn to_tokens (& self , tokens : & mut TokenStream) { if let Self :: Pointer (_ , rw) = self { tokens . append_all (match rw { AccessLevel :: RW => quote ! { * mut } , AccessLevel :: R => quote ! { * const } , }) } tokens . append_all (self . to_string () . parse :: < TokenStream > () . expect ("invalid syntax") ,) } }
};
}
