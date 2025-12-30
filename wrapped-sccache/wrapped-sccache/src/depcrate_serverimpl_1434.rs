// Generated macro for impl_1434 (impl)
macro_rules! Depcrate_serverimpl_1434 {
() => {
// Module: crate::server
// Provides: {"impl_1434"}
// Dependencies: {}
impl < I : AsyncRead + AsyncWrite + Unpin > Stream for SccacheTransport < I > { type Item = Result < Message < Request , Body < () > > > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Pin :: new (& mut self . inner) . poll_next (cx) . map (| r | r . map (| s | s . map (Message :: WithoutBody))) } }
};
}
