// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [tokio :: test] async fn get_and_set_value () { let addr = run_in_background () . await ; let mut client = make_client (addr) . await . unwrap () ; let mut stream = client . subscribe (SubscribeRequest { }) . await . unwrap () . into_inner () ; let key = "foo" . to_string () ; let value = vec ! [1_u8 , 3 , 3 , 7] ; let status = client . get (GetRequest { key : key . clone () }) . await . unwrap_err () ; assert_eq ! (status . code () , Code :: NotFound) ; client . set (SetRequest { key : key . clone () , value : value . clone () , }) . await . unwrap () ; let server_value = client . get (GetRequest { key : key . clone () }) . await . unwrap () . into_inner () . value ; assert_eq ! (value , server_value) ; let streamed_key = tokio :: time :: timeout (Duration :: from_millis (100) , stream . next ()) . await . unwrap () . unwrap () . unwrap () . key ; assert_eq ! (streamed_key , "foo") ; } async fn run_in_background () -> SocketAddr { let listener = TcpListener :: bind ("127.0.0.1:0") . await . expect ("Could not bind ephemeral socket") ; let addr = listener . local_addr () . unwrap () ; eprintln ! ("Listening on {}" , addr) ; tokio :: spawn (async move { serve_forever (listener) . await . unwrap () ; }) ; addr } }
};
}
