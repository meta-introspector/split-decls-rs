// Generated macro for can_round_trip_psk_identity (function)
macro_rules! Depcrate_msgs_handshake_testcan_round_trip_psk_identity {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"can_round_trip_psk_identity"}
// Dependencies: {}
# [test] fn can_round_trip_psk_identity () { let bytes = [0 , 1 , 0x99 , 0x11 , 0x22 , 0x33 , 0x44] ; let psk_id = PresharedKeyIdentity :: read (& mut Reader :: init (& bytes)) . unwrap () ; println ! ("{psk_id:?}") ; assert_eq ! (psk_id . obfuscated_ticket_age , 0x11223344) ; assert_eq ! (psk_id . get_encoding () , bytes . to_vec ()) ; let bytes = [0 , 5 , 0x1 , 0x2 , 0x3 , 0x4 , 0x5 , 0x11 , 0x22 , 0x33 , 0x44] ; let psk_id = PresharedKeyIdentity :: read (& mut Reader :: init (& bytes)) . unwrap () ; println ! ("{psk_id:?}") ; assert_eq ! (psk_id . identity . 0 , vec ! [0x1 , 0x2 , 0x3 , 0x4 , 0x5]) ; assert_eq ! (psk_id . obfuscated_ticket_age , 0x11223344) ; assert_eq ! (psk_id . get_encoding () , bytes . to_vec ()) ; }
};
}
