// Generated macro for sample_new_session_ticket_payload (function)
macro_rules! Depcrate_msgs_handshake_testsample_new_session_ticket_payload {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"sample_new_session_ticket_payload"}
// Dependencies: {}
fn sample_new_session_ticket_payload () -> NewSessionTicketPayload { NewSessionTicketPayload { lifetime_hint : Duration :: from_secs (1234) , ticket : Arc :: new (PayloadU16 :: new (vec ! [1 , 2 , 3])) , } }
};
}
