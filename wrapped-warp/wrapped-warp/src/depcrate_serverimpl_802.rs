// Generated macro for impl_802 (impl)
macro_rules! Depcrate_serverimpl_802 {
() => {
// Module: crate::server
// Provides: {"impl_802"}
// Dependencies: {}
impl < F , A , R > Server < F , A , R > where F : Filter + Clone + Send + Sync + 'static , < F :: Future as TryFuture > :: Ok : Reply , < F :: Future as TryFuture > :: Error : IsReject , A : accept :: Accept , R : run :: Run , { # [cfg (feature = "tls")] pub fn tls (self) -> Server < F , accept :: Tls < A > , R > { } # [doc = " Add graceful shutdown support to this server."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # async fn ex(addr: std::net::SocketAddr) {"] # [doc = " # use warp::Filter;"] # [doc = " # let filter = warp::any().map(|| \"ok\");"] # [doc = " warp::serve(filter)"] # [doc = "     .bind(addr).await"] # [doc = "     .graceful(async {"] # [doc = "         // some signal in here, such as ctrl_c"] # [doc = "     })"] # [doc = "     .run().await;"] # [doc = " # }"] # [doc = " ```"] pub fn graceful < Fut > (self , shutdown_signal : Fut) -> Server < F , A , run :: Graceful < Fut > > where Fut : Future < Output = () > + Send + 'static , { Server { acceptor : self . acceptor , filter : self . filter , pipeline : self . pipeline , runner : run :: Graceful (shutdown_signal) , } } # [doc = " Run this server."] pub async fn run (self) { R :: run (self) . await ; } }
};
}
