// Generated macro for impl_521 (impl)
macro_rules! Depcrate_reconnectimpl_521 {
() => {
// Module: crate::reconnect
// Provides: {"impl_521"}
// Dependencies: {}
impl < M , Target > Reconnect < M , Target > where M : Service < Target > , { # [doc = " Lazily connect and reconnect to a [`Service`]."] pub const fn new (mk_service : M , target : Target) -> Self { Reconnect { mk_service , state : State :: Idle , target , error : None , } } # [doc = " Reconnect to a already connected [`Service`]."] pub const fn with_connection (init_conn : M :: Response , mk_service : M , target : Target) -> Self { Reconnect { mk_service , state : State :: Connected (init_conn) , target , error : None , } } }
};
}
