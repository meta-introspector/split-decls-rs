// Generated macro for impl_208 (impl)
macro_rules! Depcrate_cache_memcachedimpl_208 {
() => {
// Module: crate::cache::memcached
// Provides: {"impl_208"}
// Dependencies: {}
impl MemcachedCache { pub fn build (url : & str , username : Option < & str > , password : Option < & str > , key_prefix : & str , expiration : u32 ,) -> Result < Operator > { let mut builder = Memcached :: default () . endpoint (url) ; if let Some (username) = username { builder = builder . username (username) ; } if let Some (password) = password { builder = builder . password (password) ; } builder = builder . root (key_prefix) . default_ttl (Duration :: from_secs (expiration . into ())) ; let op = Operator :: new (builder) ? . layer (LoggingLayer :: default ()) . finish () ; Ok (op) } }
};
}
