// Generated macro for impl_468 (impl)
macro_rules! Depcrate_msgs_persistimpl_468 {
() => {
// Module: crate::msgs::persist
// Provides: {"impl_468"}
// Dependencies: {}
impl Codec < '_ > for Tls12ServerSessionValue { fn encode (& self , bytes : & mut Vec < u8 >) { self . common . encode (bytes) ; bytes . extend_from_slice (self . master_secret . as_ref ()) ; (self . extended_ms as u8) . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { Ok (Self { common : CommonServerSessionValue :: read (r) ? , master_secret : Zeroizing :: new (match r . take (48) . and_then (| slice | slice . try_into () . ok ()) { Some (array) => array , None => return Err (InvalidMessage :: MessageTooShort) , } ,) , extended_ms : matches ! (u8 :: read (r) ?, 1) , }) } }
};
}
