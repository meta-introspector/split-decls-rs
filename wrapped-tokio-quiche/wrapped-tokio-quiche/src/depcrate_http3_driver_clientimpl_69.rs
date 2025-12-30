// Generated macro for impl_69 (impl)
macro_rules! Depcrate_http3_driver_clientimpl_69 {
() => {
// Module: crate::http3::driver::client
// Provides: {"impl_69"}
// Dependencies: {}
# [allow (private_interfaces)] impl DriverHooks for ClientHooks { type Command = ClientH3Command ; type Event = ClientH3Event ; fn new (_settings : & Http3Settings) -> Self { Self { pending_requests : BTreeMap :: new () , } } fn conn_established (_driver : & mut H3Driver < Self > , qconn : & mut QuicheConnection , _handshake_info : & HandshakeInfo ,) -> H3ConnectionResult < () > { assert ! (! qconn . is_server () , "ClientH3Driver requires a client-side QUIC connection") ; Ok (()) } fn headers_received (driver : & mut H3Driver < Self > , _qconn : & mut QuicheConnection , headers : InboundHeaders ,) -> H3ConnectionResult < () > { let Some (pending_request) = driver . hooks . pending_requests . remove (& headers . stream_id) else { return Ok (()) ; } ; Self :: handle_response (driver , headers , pending_request) } fn conn_command (driver : & mut H3Driver < Self > , qconn : & mut QuicheConnection , cmd : Self :: Command ,) -> H3ConnectionResult < () > { match cmd { ClientH3Command :: Core (c) => driver . handle_core_command (qconn , c) , ClientH3Command :: ClientRequest (req) => Self :: initiate_request (driver , qconn , req) , } } }
};
}
