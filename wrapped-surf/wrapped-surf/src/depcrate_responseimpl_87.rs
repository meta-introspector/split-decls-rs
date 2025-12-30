// Generated macro for impl_87 (impl)
macro_rules! Depcrate_responseimpl_87 {
() => {
// Module: crate::response
// Provides: {"impl_87"}
// Dependencies: {}
impl BufRead for Response { # [allow (missing_doc_code_examples)] fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & '_ [u8] > > { let this = self . project () ; this . res . poll_fill_buf (cx) } fn consume (mut self : Pin < & mut Self > , amt : usize) { Pin :: new (& mut self . res) . consume (amt) } }
};
}
