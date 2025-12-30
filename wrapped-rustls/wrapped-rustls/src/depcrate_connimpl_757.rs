// Generated macro for impl_757 (impl)
macro_rules! Depcrate_connimpl_757 {
() => {
// Module: crate::conn
// Provides: {"impl_757"}
// Dependencies: {}
impl < Side : SideData > From < ConnectionCore < Side > > for UnbufferedConnectionCommon < Side > { fn from (core : ConnectionCore < Side >) -> Self { Self { core , wants_write : false , emitted_peer_closed_state : false , } } }
};
}
