// Generated macro for impl_217 (impl)
macro_rules! Depcrate_http3_driver_test_utilsimpl_217 {
() => {
// Module: crate::http3::driver::test_utils
// Provides: {"impl_217"}
// Dependencies: {}
impl DriverTestHelper < ServerHooks > { # [doc = " Sends a new client request"] pub fn peer_client_send_request (& mut self , headers : Vec < Header > , fin : bool ,) -> anyhow :: Result < u64 > { Ok (self . peer . send_request (& mut self . pipe . client , & headers , fin) ?) } # [doc = " Try to receive an event from the controller, returns an error if"] # [doc = " the receive fails"] pub fn driver_recv_core_event (& mut self) -> anyhow :: Result < H3Event > { match self . controller . event_receiver_mut () . try_recv () ? { ServerH3Event :: Core (h3_event) => Ok (h3_event) , ev => Err (anyhow :: anyhow ! ("Not a core event: {ev:?}")) , } } # [doc = " Try to receive a `ServerH3Event` from the controller's event receiver"] pub fn driver_recv_server_event (& mut self) -> anyhow :: Result < ServerH3Event > { Ok (self . controller . event_receiver_mut () . try_recv () ?) } pub fn peer_client_poll (& mut self) -> h3 :: Result < (u64 , h3 :: Event) > { self . poll_peer () } # [doc = " Send a body from the server"] pub fn peer_client_send_body (& mut self , stream_id : u64 , body : & [u8] , fin : bool ,) -> h3 :: Result < usize > { self . peer_send_body (stream_id , body , fin) } # [doc = " Receive at most `max_read` body bytes and return the read"] # [doc = " bytes as a `Vec`"] pub fn peer_client_recv_body_vec (& mut self , stream_id : u64 , max_read : usize ,) -> h3 :: Result < Vec < u8 > > { self . peer_recv_body_vec (stream_id , max_read) } }
};
}
