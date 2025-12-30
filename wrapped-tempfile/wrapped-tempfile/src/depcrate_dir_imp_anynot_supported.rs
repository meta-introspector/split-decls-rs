// Generated macro for not_supported (function)
macro_rules! Depcrate_dir_imp_anynot_supported {
() => {
// Module: crate::dir::imp::any
// Provides: {"not_supported"}
// Dependencies: {}
fn not_supported < T > (msg : & str) -> io :: Result < T > { Err (io :: Error :: new (io :: ErrorKind :: Other , msg)) }
};
}
