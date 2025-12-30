// Generated macro for impl_1435 (impl)
macro_rules! Depcrate_serverimpl_1435 {
() => {
// Module: crate::server
// Provides: {"impl_1435"}
// Dependencies: {}
impl < I : AsyncRead + AsyncWrite + Unpin > Sink < Frame < Response , Response > > for SccacheTransport < I > { type Error = Error ; fn poll_ready (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { Pin :: new (& mut self . inner) . poll_ready (cx) } fn start_send (mut self : Pin < & mut Self > , item : Frame < Response , Response >) -> Result < () > { match item { Frame :: Message { message } => Pin :: new (& mut self . inner) . start_send (message) , Frame :: Body { chunk : Some (chunk) } => Pin :: new (& mut self . inner) . start_send (chunk) , Frame :: Body { chunk : None } => Ok (()) , } } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { Pin :: new (& mut self . inner) . poll_flush (cx) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { Pin :: new (& mut self . inner) . poll_close (cx) } }
};
}
