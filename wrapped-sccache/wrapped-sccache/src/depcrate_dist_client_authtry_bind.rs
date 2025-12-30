// Generated macro for try_bind (function)
macro_rules! Depcrate_dist_client_authtry_bind {
() => {
// Module: crate::dist::client_auth
// Provides: {"try_bind"}
// Dependencies: {}
# [doc = " Try to bind a TCP stream to any of the available port out of [`VALID_PORTS`]."] async fn try_bind () -> Result < HyperBuilderWrap > { for & port in VALID_PORTS { let mut addrs = ("localhost" , port) . to_socket_addrs () . expect ("Failed to interpret localhost address to listen on") ; let addr = addrs . next () . expect ("Expected at least one address in parsed socket address") ; match TcpStream :: connect (addr) { Ok (_) => continue , Err (ref e) if e . kind () == io :: ErrorKind :: ConnectionRefused => () , Err (e) => { return Err (e) . with_context (| | format ! ("Failed to check {} is available for binding" , addr)) ; } } match HyperBuilderWrap :: try_bind (addr) . await { Ok (s) => return Ok (s) , Err (ref err) if err . source () . and_then (| err | err . downcast_ref :: < io :: Error > ()) . map (| err | err . kind () == io :: ErrorKind :: AddrInUse) . unwrap_or (false) => { continue ; } Err (e) => return Err (e) . with_context (| | format ! ("Failed to bind to {}" , addr)) , } } bail ! ("Could not bind to any valid port: ({:?})" , VALID_PORTS) }
};
}
