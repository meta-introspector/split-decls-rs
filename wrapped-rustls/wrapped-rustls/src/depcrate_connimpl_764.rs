// Generated macro for impl_764 (impl)
macro_rules! Depcrate_connimpl_764 {
() => {
// Module: crate::conn
// Provides: {"impl_764"}
// Dependencies: {}
impl InboundUnborrowedMessage { fn unborrow (locator : & Locator , msg : InboundPlainMessage < '_ >) -> Self { Self { typ : msg . typ , version : msg . version , bounds : locator . locate (msg . payload) , } } fn reborrow < 'b > (self , delocator : & Delocator < 'b >) -> InboundPlainMessage < 'b > { InboundPlainMessage { typ : self . typ , version : self . version , payload : delocator . slice_from_range (& self . bounds) , } } }
};
}
