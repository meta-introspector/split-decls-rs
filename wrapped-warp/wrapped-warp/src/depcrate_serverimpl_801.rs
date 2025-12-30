// Generated macro for impl_801 (impl)
macro_rules! Depcrate_serverimpl_801 {
() => {
// Module: crate::server
// Provides: {"impl_801"}
// Dependencies: {}
impl < F , R > Server < F , accept :: LazyTcp , R > where F : Filter + Clone + Send + Sync + 'static , < F :: Future as TryFuture > :: Ok : Reply , < F :: Future as TryFuture > :: Error : IsReject , R : run :: Run , { # [doc = " Binds and runs this server."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if we are unable to bind to the provided address."] # [doc = ""] # [doc = " To handle bind failures, bind a listener and call `incoming()`."] pub async fn run (self , addr : impl Into < SocketAddr >) { self . bind (addr) . await . run () . await ; } # [doc = " Binds this server."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if we are unable to bind to the provided address."] # [doc = ""] # [doc = " To handle bind failures, bind a listener and call `incoming()`."] pub async fn bind (self , addr : impl Into < SocketAddr >) -> Server < F , tokio :: net :: TcpListener , R > { let addr = addr . into () ; let acceptor = tokio :: net :: TcpListener :: bind (addr) . await . expect ("failed to bind to address") ; self . incoming (acceptor) } # [doc = " Configure the server with an acceptor of incoming connections."] pub fn incoming < A > (self , acceptor : A) -> Server < F , A , R > { Server { acceptor , filter : self . filter , pipeline : self . pipeline , runner : self . runner , } } }
};
}
