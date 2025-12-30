// Generated macro for impl_472 (impl)
macro_rules! Depcrate_msgs_persistimpl_472 {
() => {
// Module: crate::msgs::persist
// Provides: {"impl_472"}
// Dependencies: {}
impl Codec < '_ > for Tls13ServerSessionValue { fn encode (& self , bytes : & mut Vec < u8 >) { self . common . encode (bytes) ; self . secret . encode (bytes) ; self . age_obfuscation_offset . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { Ok (Self { common : CommonServerSessionValue :: read (r) ? , secret : Zeroizing :: new (PayloadU8 :: read (r) ?) , age_obfuscation_offset : u32 :: read (r) ? , freshness : None , }) } }
};
}
