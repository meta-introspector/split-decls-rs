// Generated macro for wrapped_dn_encoding (function)
macro_rules! Depcrate_msgs_handshake_testwrapped_dn_encoding {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"wrapped_dn_encoding"}
// Dependencies: {}
# [test] fn wrapped_dn_encoding () { let subject = b"subject" ; let dn = DistinguishedName :: in_sequence (& subject [..]) ; const DER_SEQUENCE_TAG : u8 = 0x30 ; let expected_prefix = vec ! [DER_SEQUENCE_TAG , subject . len () as u8] ; assert_eq ! (dn . as_ref () , [expected_prefix , subject . to_vec ()] . concat ()) ; }
};
}
