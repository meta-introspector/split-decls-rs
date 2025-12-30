// Generated macro for MidHandshake (enum)
macro_rules! Depcrate_common_handshakeMidHandshake {
() => {
// Module: crate::common::handshake
// Provides: {"MidHandshake"}
// Dependencies: {}
pub (crate) enum MidHandshake < IS : IoSession > { Handshaking (IS) , End , SendAlert { io : IS :: Io , alert : AcceptedAlert , error : io :: Error , } , Error { io : IS :: Io , error : io :: Error , } , }
};
}
