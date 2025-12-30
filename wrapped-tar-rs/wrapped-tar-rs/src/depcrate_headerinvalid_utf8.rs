// Generated macro for invalid_utf8 (function)
macro_rules! Depcrate_headerinvalid_utf8 {
() => {
// Module: crate::header
// Provides: {"invalid_utf8"}
// Dependencies: {}
# [cfg (target_arch = "wasm32")] fn invalid_utf8 < T > (_ : T) -> io :: Error { io :: Error :: new (io :: ErrorKind :: InvalidData , "Invalid utf-8") }
};
}
