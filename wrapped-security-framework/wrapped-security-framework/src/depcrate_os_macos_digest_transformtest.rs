// Generated macro for test (module)
macro_rules! Depcrate_os_macos_digest_transformtest {
() => {
// Module: crate::os::macos::digest_transform
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn md5 () { let data = CFData :: from_buffer ("The quick brown fox jumps over the lazy dog" . as_bytes ()) ; let hash = Builder :: new () . type_ (DigestType :: md5 ()) . execute (& data) . unwrap () ; assert_eq ! (hex :: encode (hash . bytes ()) , "9e107d9d372bb6826bd81d3542a419d6") ; } # [test] fn hmac_sha1 () { let data = CFData :: from_buffer ("The quick brown fox jumps over the lazy dog" . as_bytes ()) ; let key = CFData :: from_buffer (b"key") ; let hash = Builder :: new () . type_ (DigestType :: hmac_sha1 ()) . hmac_key (key) . execute (& data) . unwrap () ; assert_eq ! (hex :: encode (hash . bytes ()) , "de7c9b85b8b78aa6bc8a7a36f70a90701c9db4d9") ; } }
};
}
