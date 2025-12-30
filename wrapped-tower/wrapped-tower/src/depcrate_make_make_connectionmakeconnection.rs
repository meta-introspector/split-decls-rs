// Generated macro for MakeConnection (trait)
macro_rules! Depcrate_make_make_connectionMakeConnection {
() => {
// Module: crate::make::make_connection
// Provides: {"MakeConnection"}
// Dependencies: {}
# [doc = " The [`MakeConnection`] trait is used to create transports."] # [doc = ""] # [doc = " The goal of this service is to allow composable methods for creating"] # [doc = " `AsyncRead + AsyncWrite` transports. This could mean creating a TLS"] # [doc = " based connection or using some other method to authenticate the connection."] pub trait MakeConnection < Target > : Sealed < (Target ,) > { # [doc = " The transport provided by this service"] type Connection : AsyncRead + AsyncWrite ; # [doc = " Errors produced by the connecting service"] type Error ; # [doc = " The future that eventually produces the transport"] type Future : Future < Output = Result < Self :: Connection , Self :: Error > > ; # [doc = " Returns `Poll::Ready(Ok(()))` when it is able to make more connections."] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > ; # [doc = " Connect and return a transport asynchronously"] fn make_connection (& mut self , target : Target) -> Self :: Future ; }
};
}
