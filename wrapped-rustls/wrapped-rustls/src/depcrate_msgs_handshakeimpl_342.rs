// Generated macro for impl_342 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_342 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_342"}
// Dependencies: {}
impl ServerDhParams { pub (crate) fn new (kx : & dyn ActiveKeyExchange) -> Self { let Some (params) = kx . ffdhe_group () else { panic ! ("invalid NamedGroup for DHE key exchange: {:?}" , kx . group ()) ; } ; Self { dh_p : PayloadU16 :: new (params . p . to_vec ()) , dh_g : PayloadU16 :: new (params . g . to_vec ()) , dh_ys : PayloadU16 :: new (kx . pub_key () . to_vec ()) , } } pub (crate) fn as_ffdhe_group (& self) -> FfdheGroup < '_ > { FfdheGroup :: from_params_trimming_leading_zeros (& self . dh_p . 0 , & self . dh_g . 0) } }
};
}
