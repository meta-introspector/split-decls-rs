// Generated macro for ToTokenStream (trait)
macro_rules! Depcrate_to_tokensToTokenStream {
() => {
// Module: crate::to_tokens
// Provides: {"ToTokenStream"}
// Dependencies: {}
# [doc = " Turn a type into a [`TokenStream`]."] pub (crate) trait ToTokenStream : Sized { fn append_to (self , ts : & mut TokenStream) ; fn into_token_stream (self) -> TokenStream { let mut ts = TokenStream :: new () ; self . append_to (& mut ts) ; ts } }
};
}
