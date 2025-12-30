// Generated macro for impl_112 (impl)
macro_rules! Depcrate_middleware_redirectimpl_112 {
() => {
// Module: crate::middleware::redirect
// Provides: {"impl_112"}
// Dependencies: {}
# [async_trait :: async_trait] impl Middleware for Redirect { # [allow (missing_doc_code_examples)] async fn handle (& self , mut req : Request , client : Client , next : Next < '_ >) -> Result < Response > { let mut redirect_count : u8 = 0 ; let mut base_url = req . url () . clone () ; while redirect_count < self . attempts { redirect_count += 1 ; let r : Request = req . clone () ; let res : Response = client . send (r) . await ? ; if REDIRECT_CODES . contains (& res . status ()) { if let Some (location) = res . header (headers :: LOCATION) { let http_req : & mut http :: Request = req . as_mut () ; * http_req . url_mut () = match Url :: parse (location . last () . as_str ()) { Ok (valid_url) => { base_url = valid_url ; base_url . clone () } Err (e) => match e { http :: url :: ParseError :: RelativeUrlWithoutBase => { base_url . join (location . last () . as_str ()) ? } e => return Err (e . into ()) , } , } ; } } else { break ; } } Ok (next . run (req , client) . await ?) } }
};
}
