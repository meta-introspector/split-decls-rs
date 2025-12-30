// Generated macro for impl_134 (impl)
macro_rules! Depcrate_serverimpl_134 {
() => {
// Module: crate::server
// Provides: {"impl_134"}
// Dependencies: {}
impl < IO > Accept < IO > { # [inline] pub fn into_fallible (self) -> FallibleAccept < IO > { FallibleAccept (self . 0) } pub fn get_ref (& self) -> Option < & IO > { match & self . 0 { MidHandshake :: Handshaking (sess) => Some (sess . get_ref () . 0) , MidHandshake :: SendAlert { io , .. } => Some (io) , MidHandshake :: Error { io , .. } => Some (io) , MidHandshake :: End => None , } } pub fn get_mut (& mut self) -> Option < & mut IO > { match & mut self . 0 { MidHandshake :: Handshaking (sess) => Some (sess . get_mut () . 0) , MidHandshake :: SendAlert { io , .. } => Some (io) , MidHandshake :: Error { io , .. } => Some (io) , MidHandshake :: End => None , } } }
};
}
