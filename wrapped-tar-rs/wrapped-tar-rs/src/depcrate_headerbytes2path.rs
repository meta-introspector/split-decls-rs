// Generated macro for bytes2path (function)
macro_rules! Depcrate_headerbytes2path {
() => {
// Module: crate::header
// Provides: {"bytes2path"}
// Dependencies: {}
# [cfg (target_arch = "wasm32")] pub fn bytes2path (bytes : Cow < [u8] >) -> io :: Result < Cow < Path > > { Ok (match bytes { Cow :: Borrowed (bytes) => { Cow :: Borrowed (Path :: new (str :: from_utf8 (bytes) . map_err (invalid_utf8) ?)) } Cow :: Owned (bytes) => Cow :: Owned (PathBuf :: from (String :: from_utf8 (bytes) . map_err (invalid_utf8) ? ,)) , }) }
};
}
