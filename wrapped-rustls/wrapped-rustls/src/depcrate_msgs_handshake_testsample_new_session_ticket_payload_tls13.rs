// Generated macro for sample_new_session_ticket_payload_tls13 (function)
macro_rules! Depcrate_msgs_handshake_testsample_new_session_ticket_payload_tls13 {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"sample_new_session_ticket_payload_tls13"}
// Dependencies: {}
fn sample_new_session_ticket_payload_tls13 () -> NewSessionTicketPayloadTls13 { NewSessionTicketPayloadTls13 { lifetime : Duration :: from_secs (123) , age_add : 1234 , nonce : PayloadU8 :: new (vec ! [1 , 2 , 3]) , ticket : Arc :: new (PayloadU16 :: new (vec ! [4 , 5 , 6])) , extensions : NewSessionTicketExtensions { max_early_data_size : Some (1234) , } , } }
};
}
