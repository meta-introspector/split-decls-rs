// Generated macro for impl_16 (impl)
macro_rules! Depcrate_attributeimpl_16 {
() => {
// Module: crate::attribute
// Provides: {"impl_16"}
// Dependencies: {}
impl ToTokens for Formatter { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { tokens . extend (match self { Formatter :: Json => quote ! { . json () } , Formatter :: Pretty => quote ! { . pretty () } , }) } }
};
}
