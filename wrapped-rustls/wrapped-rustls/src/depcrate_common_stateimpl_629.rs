// Generated macro for impl_629 (impl)
macro_rules! Depcrate_common_stateimpl_629 {
() => {
// Module: crate::common_state
// Provides: {"impl_629"}
// Dependencies: {}
impl < 'a , const TLS13 : bool > HandshakeFlight < 'a , TLS13 > { pub (crate) fn new (transcript : & 'a mut HandshakeHash) -> Self { Self { transcript , body : Vec :: new () , } } pub (crate) fn add (& mut self , hs : HandshakeMessagePayload < '_ >) { let start_len = self . body . len () ; hs . encode (& mut self . body) ; self . transcript . add (& self . body [start_len ..]) ; } pub (crate) fn finish (self , common : & mut CommonState) { common . send_msg (Message { version : match TLS13 { true => ProtocolVersion :: TLSv1_3 , false => ProtocolVersion :: TLSv1_2 , } , payload : MessagePayload :: HandshakeFlight (Payload :: new (self . body)) , } , TLS13 ,) ; } }
};
}
