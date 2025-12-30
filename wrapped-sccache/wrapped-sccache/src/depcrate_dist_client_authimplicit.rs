// Generated macro for implicit (module)
macro_rules! Depcrate_dist_client_authimplicit {
() => {
// Module: crate::dist::client_auth
// Provides: {"implicit"}
// Dependencies: {}
mod implicit { use super :: { MIN_TOKEN_VALIDITY , MIN_TOKEN_VALIDITY_WARNING , REDIRECT_WITH_AUTH_JSON , html_response , json_response , query_pairs , } ; use bytes :: Bytes ; use futures :: channel :: oneshot ; use http_body_util :: Full ; use hyper :: { Method , Request , Response , StatusCode } ; use std :: collections :: HashMap ; use std :: sync :: Mutex ; use std :: sync :: mpsc ; use std :: time :: { Duration , Instant } ; use url :: Url ; use crate :: errors :: * ; const CLIENT_ID_PARAM : & str = "client_id" ; const REDIRECT_PARAM : & str = "redirect_uri" ; const RESPONSE_TYPE_PARAM : & str = "response_type" ; const RESPONSE_TYPE_PARAM_VALUE : & str = "token" ; const STATE_PARAM : & str = "state" ; const TOKEN_RESULT_PARAM : & str = "access_token" ; const TOKEN_TYPE_RESULT_PARAM : & str = "token_type" ; const TOKEN_TYPE_RESULT_PARAM_VALUE : & str = "bearer" ; const EXPIRES_IN_RESULT_PARAM : & str = "expires_in" ; const STATE_RESULT_PARAM : & str = "state" ; pub struct State { pub auth_url : String , pub auth_state_value : String , pub token_tx : mpsc :: SyncSender < String > , pub shutdown_tx : Option < oneshot :: Sender < () > > , } pub static STATE : Mutex < Option < State > > = Mutex :: new (None) ; pub fn finish_url (client_id : & str , url : & mut Url , redirect_uri : & str , state : & str) { url . query_pairs_mut () . append_pair (CLIENT_ID_PARAM , client_id) . append_pair (REDIRECT_PARAM , redirect_uri) . append_pair (RESPONSE_TYPE_PARAM , RESPONSE_TYPE_PARAM_VALUE) . append_pair (STATE_PARAM , state) ; } fn handle_response (params : HashMap < String , String >) -> Result < (String , Instant , String) > { let token = params . get (TOKEN_RESULT_PARAM) . context ("No token found in response") ? ; let bearer = params . get (TOKEN_TYPE_RESULT_PARAM) . context ("No token type found in response") ? ; if bearer . to_lowercase () != TOKEN_TYPE_RESULT_PARAM_VALUE { bail ! ("Token type in response is not {}" , TOKEN_TYPE_RESULT_PARAM_VALUE) } let expires_in = params . get (EXPIRES_IN_RESULT_PARAM) . context ("No expiry found in response") ? ; let expires_at = Instant :: now () + Duration :: from_secs (expires_in . parse () . map_err (| _ | anyhow ! ("Failed to parse expiry as integer")) ? ,) ; let state = params . get (STATE_RESULT_PARAM) . context ("No state found in response") ? ; Ok ((token . to_owned () , expires_at , state . to_owned ())) } const SAVE_AUTH_AFTER_REDIRECT : & str = r##"<!doctype html>
    <html lang="en">
    <head><meta charset="utf-8"></head>
    <body>
        <script>
        function writemsg(m) {
            document.body.appendChild(document.createTextNode(m.toString()));
            document.body.appendChild(document.createElement('br'));
        }
        function go() {
            writemsg('Saving authentication details...');
            var qs = window.location.hash.slice(1);
            if (qs.length === 0) {
                writemsg("ERROR: No URL hash returned from authorizer");
                return
            }
            fetch('/save_auth?' + qs, { method: 'POST' }).then(function (response) {
                if (!response.ok) {
                    throw 'Error during saving authentication - ' + response.status + ': ' + response.statusText;
                }
                writemsg('Authentication complete, you can now close this page!');
            }).catch(writemsg);
        }
        go();
        </script>
    </body>
    </html>
    "## ; pub fn serve (req : Request < hyper :: body :: Incoming >) -> Result < Response < Full < Bytes > > > { let mut state = STATE . lock () . unwrap () ; let state = state . as_mut () . unwrap () ; debug ! ("Handling {} {}" , req . method () , req . uri ()) ; let response = match (req . method () , req . uri () . path ()) { (& Method :: GET , "/") => html_response (REDIRECT_WITH_AUTH_JSON) , (& Method :: GET , "/auth_detail.json") => json_response (& state . auth_url) ? , (& Method :: GET , "/redirect") => html_response (SAVE_AUTH_AFTER_REDIRECT) , (& Method :: POST , "/save_auth") => { let query_pairs = query_pairs (& req . uri () . to_string ()) ? ; let (token , expires_at , auth_state) = handle_response (query_pairs) . context ("Failed to save auth after redirect") ? ; if auth_state != state . auth_state_value { return Err (anyhow ! ("Mismatched auth states after redirect")) ; } if expires_at - Instant :: now () < MIN_TOKEN_VALIDITY { warn ! ("Token retrieved expires in under {}" , MIN_TOKEN_VALIDITY_WARNING) ; eprintln ! ("sccache: Token retrieved expires in under {}" , MIN_TOKEN_VALIDITY_WARNING) ; } state . token_tx . send (token) . unwrap () ; state . shutdown_tx . take () . unwrap () . send (()) . unwrap () ; json_response (& "") ? } _ => { warn ! ("Route not found") ; Response :: builder () . status (StatusCode :: NOT_FOUND) . body ("" . into ()) . unwrap () } } ; Ok (response) } }
};
}
