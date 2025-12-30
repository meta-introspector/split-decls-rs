// Generated macro for hash (function)
macro_rules! Depcrate_md5hash {
() => {
// Module: crate::md5
// Provides: {"hash"}
// Dependencies: {}
# [cfg (feature = "v3")] pub (crate) fn hash (ns : & [u8] , src : & [u8]) -> [u8 ; 16] { use md5 :: { Digest , Md5 } ; let mut hasher = Md5 :: new () ; hasher . update (ns) ; hasher . update (src) ; let mut bytes = [0 ; 16] ; bytes . copy_from_slice (& hasher . finalize () [.. 16]) ; bytes }
};
}
