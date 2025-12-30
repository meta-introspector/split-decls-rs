// Generated macro for path2bytes (function)
macro_rules! Depcrate_utilpath2bytes {
() => {
// Module: crate::util
// Provides: {"path2bytes"}
// Dependencies: {}
# [cfg (windows)] pub fn path2bytes (p : & Path) -> Result < Cow < [u8] > , Error > { p . to_str () . map (| s | s . as_bytes ()) . ok_or_else (| | { Error :: new (ErrorCode :: Session (raw :: LIBSSH2_ERROR_INVAL) , "only unicode paths on windows may be used" ,) }) . map (| bytes | { if bytes . contains (& b'\\') { let mut bytes = bytes . to_owned () ; for b in & mut bytes { if * b == b'\\' { * b = b'/' ; } } Cow :: Owned (bytes) } else { Cow :: Borrowed (bytes) } }) . and_then (check) }
};
}
