// Generated macro for can_round_trip_psk_offer (function)
macro_rules! Depcrate_msgs_handshake_testcan_round_trip_psk_offer {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"can_round_trip_psk_offer"}
// Dependencies: {}
# [test] fn can_round_trip_psk_offer () { let bytes = [0 , 7 , 0 , 1 , 0x99 , 0x11 , 0x22 , 0x33 , 0x44 , 0 , 4 , 3 , 0x01 , 0x02 , 0x3 ,] ; let psko = PresharedKeyOffer :: read (& mut Reader :: init (& bytes)) . unwrap () ; println ! ("{psko:?}") ; assert_eq ! (psko . identities . len () , 1) ; assert_eq ! (psko . identities [0] . identity . 0 , vec ! [0x99]) ; assert_eq ! (psko . identities [0] . obfuscated_ticket_age , 0x11223344) ; assert_eq ! (psko . binders . len () , 1) ; assert_eq ! (psko . binders [0] . as_ref () , & [1 , 2 , 3]) ; assert_eq ! (psko . get_encoding () , bytes . to_vec ()) ; }
};
}
