// Generated macro for impl_trait (function)
macro_rules! Depcrate_deriveimpl_trait {
() => {
// Module: crate::derive
// Provides: {"impl_trait"}
// Dependencies: {}
fn impl_trait (name : & proc_macro2 :: Ident , into_arms : TokenStream2 , from_arms : TokenStream2 ,) -> TokenStream2 { quote ! { unsafe impl :: tracing_forest :: Tag for # name { fn as_field (& self) -> u64 { match * self { # into_arms } } fn from_field (value : u64) -> :: tracing_forest :: tag :: TagData { match value { # from_arms _ => panic ! ("A tag type was set, but an unrecognized tag was sent: {}. Make sure you're using the same tag type, and that you're not using `__event_tag` as a field name for anything except tags." , value) , } } } } }
};
}
