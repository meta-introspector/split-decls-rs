// Generated macro for handle_ebadf (function)
macro_rules! Depcrate_io_stdiohandle_ebadf {
() => {
// Module: crate::io::stdio
// Provides: {"handle_ebadf"}
// Dependencies: {}
fn handle_ebadf < T > (r : io :: Result < T > , default : impl FnOnce () -> io :: Result < T >) -> io :: Result < T > { match r { Err (ref e) if stdio :: is_ebadf (e) => default () , r => r , } }
};
}
