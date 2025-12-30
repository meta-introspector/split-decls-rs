// Generated macro for impl_255 (impl)
macro_rules! Depcrate_cache_webdavimpl_255 {
() => {
// Module: crate::cache::webdav
// Provides: {"impl_255"}
// Dependencies: {}
impl WebdavCache { # [doc = " Create a new `WebdavCache`."] pub fn build (endpoint : & str , key_prefix : & str , username : Option < & str > , password : Option < & str > , token : Option < & str > ,) -> Result < Operator > { let builder = Webdav :: default () . endpoint (endpoint) . root (key_prefix) . username (username . unwrap_or_default ()) . password (password . unwrap_or_default ()) . token (token . unwrap_or_default ()) ; let op = Operator :: new (builder) ? . layer (HttpClientLayer :: new (set_user_agent ())) . layer (LoggingLayer :: default ()) . finish () ; Ok (op) } }
};
}
