// Generated macro for get_token_oauth2_implicit (function)
macro_rules! Depcrate_dist_client_authget_token_oauth2_implicit {
() => {
// Module: crate::dist::client_auth
// Provides: {"get_token_oauth2_implicit"}
// Dependencies: {}
pub fn get_token_oauth2_implicit (client_id : & str , mut auth_url : Url) -> Result < String > { let runtime = Runtime :: new () ? ; let mut server = runtime . block_on (async move { try_bind () . await }) ? ; let port = server . local_addr () . port () ; let _guard = runtime . enter () ; let handle = runtime . spawn (async move { server . serve (implicit :: serve) . await . unwrap () ; }) ; let redirect_uri = format ! ("http://localhost:{}/redirect" , port) ; let auth_state_value = Uuid :: new_v4 () . as_simple () . to_string () ; implicit :: finish_url (client_id , & mut auth_url , & redirect_uri , & auth_state_value) ; info ! ("Listening on http://localhost:{} with 1 thread." , port) ; println ! ("sccache: Please visit http://localhost:{} in your browser" , port) ; let (shutdown_tx , shutdown_rx) = oneshot :: channel () ; let (token_tx , token_rx) = mpsc :: sync_channel (1) ; let state = implicit :: State { auth_url : auth_url . to_string () , auth_state_value , token_tx , shutdown_tx : Some (shutdown_tx) , } ; * implicit :: STATE . lock () . unwrap () = Some (state) ; runtime . block_on (async move { if let Err (e) = shutdown_rx . await { warn ! ("Something went wrong while waiting for auth server shutdown: {}" , e) ; } }) ; handle . abort () ; info ! ("Server finished, returning token") ; Ok (token_rx . try_recv () . expect ("Hyper shutdown but token not available - internal error")) }
};
}
