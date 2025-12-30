// Generated macro for impl_286 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_286 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_286"}
// Dependencies: {}
impl < 'a > Codec < 'a > for ClientSessionTicket { fn encode (& self , bytes : & mut Vec < u8 >) { match self { Self :: Request => () , Self :: Offer (p) => p . encode (bytes) , } } fn read (r : & mut Reader < 'a >) -> Result < Self , InvalidMessage > { Ok (match r . left () { 0 => Self :: Request , _ => Self :: Offer (Payload :: read (r) . into_owned ()) , }) } }
};
}
