// Generated macro for call_with_last_error (function)
macro_rules! Depcrate_verification_windowscall_with_last_error {
() => {
// Module: crate::verification::windows
// Provides: {"call_with_last_error"}
// Dependencies: {}
fn call_with_last_error < T , F : FnMut () -> Option < T > > (mut call : F) -> Result < T , TlsError > { if let Some (res) = call () { Ok (res) } else { Err (TlsError :: General (std :: io :: Error :: last_os_error () . to_string () ,)) } }
};
}
