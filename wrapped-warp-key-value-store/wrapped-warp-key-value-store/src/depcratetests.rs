// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use tokio :: net :: TcpListener ; # [tokio :: test] async fn get_and_set_value () { let addr = run_in_background () . await ; let client = reqwest :: Client :: builder () . gzip (true) . build () . unwrap () ; let response = client . get (& format ! ("http://{}/foo" , addr)) . send () . await . unwrap () ; assert_eq ! (response . status () , StatusCode :: NOT_FOUND) ; let response = client . post (& format ! ("http://{}/foo" , addr)) . body ("Hello, World!") . send () . await . unwrap () ; assert_eq ! (response . status () , StatusCode :: OK) ; let response = client . get (& format ! ("http://{}/foo" , addr)) . send () . await . unwrap () ; assert_eq ! (response . status () , StatusCode :: OK) ; let body = response . text () . await . unwrap () ; assert_eq ! (body , "Hello, World!") ; } async fn run_in_background () -> SocketAddr { let listener = TcpListener :: bind ("127.0.0.1:0") . await . expect ("Could not bind ephemeral socket") ; let addr = listener . local_addr () . unwrap () ; eprintln ! ("Listening on {}" , addr) ; tokio :: spawn (async move { serve_forever (listener) . await . unwrap () ; }) ; addr } }
};
}
