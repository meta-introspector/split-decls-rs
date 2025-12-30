// Generated macro for impl_390 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_390 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_390"}
// Dependencies: {}
impl Codec < '_ > for EchConfigContents { fn encode (& self , bytes : & mut Vec < u8 >) { self . key_config . encode (bytes) ; self . maximum_name_length . encode (bytes) ; let dns_name = & self . public_name . borrow () ; PayloadU8 :: < MaybeEmpty > :: encode_slice (dns_name . as_ref () . as_ref () , bytes) ; self . extensions . encode (bytes) ; } fn read (r : & mut Reader < '_ >) -> Result < Self , InvalidMessage > { Ok (Self { key_config : HpkeKeyConfig :: read (r) ? , maximum_name_length : u8 :: read (r) ? , public_name : { DnsName :: try_from (PayloadU8 :: < MaybeEmpty > :: read (r) ? . 0 . as_slice () ,) . map_err (| _ | InvalidMessage :: InvalidServerName) ? . to_owned () } , extensions : Vec :: read (r) ? , }) } }
};
}
