// Generated macro for url_origin (function)
macro_rules! Depcrate_originurl_origin {
() => {
// Module: crate::origin
// Provides: {"url_origin"}
// Dependencies: {}
pub fn url_origin (url : & Url) -> Origin { let scheme = url . scheme () ; match scheme { "blob" => { let result = Url :: parse (url . path ()) ; match result { Ok (ref url) => url_origin (url) , Err (_) => Origin :: new_opaque () , } } "ftp" | "http" | "https" | "ws" | "wss" => Origin :: Tuple (scheme . to_owned () , url . host () . unwrap () . to_owned () , url . port_or_known_default () . unwrap () ,) , "file" => Origin :: new_opaque () , _ => Origin :: new_opaque () , } }
};
}
