// Generated macro for impl_1399 (impl)
macro_rules! Depcrate_serverimpl_1399 {
() => {
// Module: crate::server
// Provides: {"impl_1399"}
// Dependencies: {}
impl < C : CommandCreatorSync > SccacheServer < tokio :: net :: TcpListener , C > { pub fn new (port : u16 , runtime : Runtime , client : Client , dist_client : DistClientContainer , storage : Arc < dyn Storage > ,) -> Result < Self > { let addr = crate :: net :: SocketAddr :: with_port (port) ; let listener = runtime . block_on (tokio :: net :: TcpListener :: bind (addr . as_net () . unwrap ())) ? ; Ok (Self :: with_listener (listener , runtime , client , dist_client , storage ,)) } }
};
}
