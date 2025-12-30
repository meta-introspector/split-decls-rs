// Generated macro for UnbufferedConnectionCommon (struct)
macro_rules! Depcrate_connUnbufferedConnectionCommon {
() => {
// Module: crate::conn
// Provides: {"UnbufferedConnectionCommon"}
// Dependencies: {}
# [doc = " Interface shared by unbuffered client and server connections."] pub struct UnbufferedConnectionCommon < Side : SideData > { pub (crate) core : ConnectionCore < Side > , wants_write : bool , emitted_peer_closed_state : bool , }
};
}
