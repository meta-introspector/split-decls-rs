// Generated macro for impl_153 (impl)
macro_rules! Depcrate_eitherimpl_153 {
() => {
// Module: crate::either
// Provides: {"impl_153"}
// Dependencies: {}
impl < L , R > AsyncBufRead for Either < L , R > where L : AsyncBufRead , R : AsyncBufRead , { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < & [u8] > > { delegate_call ! (self . poll_fill_buf (cx)) } fn consume (self : Pin < & mut Self > , amt : usize) { delegate_call ! (self . consume (amt)) ; } }
};
}
