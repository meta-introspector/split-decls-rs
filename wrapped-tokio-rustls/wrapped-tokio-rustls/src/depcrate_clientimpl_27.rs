// Generated macro for impl_27 (impl)
macro_rules! Depcrate_clientimpl_27 {
() => {
// Module: crate::client
// Provides: {"impl_27"}
// Dependencies: {}
impl < IO > Connect < IO > { # [inline] pub fn into_fallible (self) -> FallibleConnect < IO > { FallibleConnect (self . 0) } pub fn get_ref (& self) -> Option < & IO > { match & self . 0 { MidHandshake :: Handshaking (sess) => Some (sess . get_ref () . 0) , MidHandshake :: SendAlert { io , .. } => Some (io) , MidHandshake :: Error { io , .. } => Some (io) , MidHandshake :: End => None , } } pub fn get_mut (& mut self) -> Option < & mut IO > { match & mut self . 0 { MidHandshake :: Handshaking (sess) => Some (sess . get_mut () . 0) , MidHandshake :: SendAlert { io , .. } => Some (io) , MidHandshake :: Error { io , .. } => Some (io) , MidHandshake :: End => None , } } }
};
}
