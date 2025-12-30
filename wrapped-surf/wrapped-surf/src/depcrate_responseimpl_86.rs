// Generated macro for impl_86 (impl)
macro_rules! Depcrate_responseimpl_86 {
() => {
// Module: crate::response
// Provides: {"impl_86"}
// Dependencies: {}
impl AsyncRead for Response { # [allow (missing_doc_code_examples)] fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < Result < usize , io :: Error > > { Pin :: new (& mut self . res) . poll_read (cx , buf) } }
};
}
