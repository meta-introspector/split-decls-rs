// Generated macro for impl_465 (impl)
macro_rules! Depcrate_msgs_persistimpl_465 {
() => {
// Module: crate::msgs::persist
// Provides: {"impl_465"}
// Dependencies: {}
impl Codec < '_ > for ServerSessionValue { fn encode (& self , bytes : & mut Vec < u8 >) { match self { Self :: Tls12 (value) => { ProtocolVersion :: TLSv1_2 . encode (bytes) ; value . encode (bytes) ; } Self :: Tls13 (value) => { ProtocolVersion :: TLSv1_3 . encode (bytes) ; value . encode (bytes) ; } } } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { match ProtocolVersion :: read (r) ? { ProtocolVersion :: TLSv1_2 => Ok (Self :: Tls12 (Tls12ServerSessionValue :: read (r) ?)) , ProtocolVersion :: TLSv1_3 => Ok (Self :: Tls13 (Tls13ServerSessionValue :: read (r) ?)) , _ => Err (InvalidMessage :: UnknownProtocolVersion) , } } }
};
}
