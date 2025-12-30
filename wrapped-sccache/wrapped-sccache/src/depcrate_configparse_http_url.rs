// Generated macro for parse_http_url (function)
macro_rules! Depcrate_configparse_http_url {
() => {
// Module: crate::config
// Provides: {"parse_http_url"}
// Dependencies: {}
# [cfg (any (feature = "dist-client" , feature = "dist-server"))] fn parse_http_url (url : & str) -> Result < reqwest :: Url > { use std :: net :: SocketAddr ; let url = if let Ok (sa) = url . parse :: < SocketAddr > () { warn ! ("Url {} has no scheme, assuming http" , url) ; reqwest :: Url :: parse (& format ! ("http://{}" , sa)) } else { reqwest :: Url :: parse (url) } ? ; if url . scheme () != "http" && url . scheme () != "https" { bail ! ("url not http or https") } if url . path () != "/" { bail ! ("url has a relative path (currently unsupported)") } Ok (url) }
};
}
