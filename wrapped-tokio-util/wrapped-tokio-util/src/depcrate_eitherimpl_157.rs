// Generated macro for impl_157 (impl)
macro_rules! Depcrate_eitherimpl_157 {
() => {
// Module: crate::either
// Provides: {"impl_157"}
// Dependencies: {}
impl < L , R , Item , Error > futures_sink :: Sink < Item > for Either < L , R > where L : futures_sink :: Sink < Item , Error = Error > , R : futures_sink :: Sink < Item , Error = Error > , { type Error = Error ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < std :: result :: Result < () , Self :: Error > > { delegate_call ! (self . poll_ready (cx)) } fn start_send (self : Pin < & mut Self > , item : Item) -> std :: result :: Result < () , Self :: Error > { delegate_call ! (self . start_send (item)) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < std :: result :: Result < () , Self :: Error > > { delegate_call ! (self . poll_flush (cx)) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < std :: result :: Result < () , Self :: Error > > { delegate_call ! (self . poll_close (cx)) } }
};
}
