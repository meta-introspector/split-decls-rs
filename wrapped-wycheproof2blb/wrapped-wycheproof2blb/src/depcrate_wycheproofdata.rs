// Generated macro for data (function)
macro_rules! Depcrate_wycheproofdata {
() => {
// Module: crate::wycheproof
// Provides: {"data"}
// Dependencies: {}
# [doc = " Retrieve Wycheproof test vectors from the given filename in a Wycheproof repo."] pub fn data (wycheproof_dir : & str , filename : & str) -> Vec < u8 > { let path = std :: path :: Path :: new (& wycheproof_dir) . join ("testvectors") . join (filename) ; std :: fs :: read (& path) . unwrap_or_else (| _ | panic ! ("Test vector file {filename} not found at {path:?}")) }
};
}
