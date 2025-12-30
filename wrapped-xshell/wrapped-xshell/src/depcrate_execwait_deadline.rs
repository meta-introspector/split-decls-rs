// Generated macro for wait_deadline (function)
macro_rules! Depcrate_execwait_deadline {
() => {
// Module: crate::exec
// Provides: {"wait_deadline"}
// Dependencies: {}
pub (crate) fn wait_deadline (child : & mut Child , deadline : Option < Instant > ,) -> io :: Result < ExitStatus > { let Some (deadline) = deadline else { return child . wait () ; } ; let mut sleep_ms = 1 ; let sleep_ms_max = 64 ; loop { match child . try_wait () ? { Some (status) => return Ok (status) , None => { } } if Instant :: now () > deadline { let _ = child . kill () ; let _ = child . wait () ; return Err (io :: ErrorKind :: TimedOut . into ()) ; } std :: thread :: sleep (Duration :: from_millis (sleep_ms)) ; sleep_ms = std :: cmp :: min (sleep_ms * 2 , sleep_ms_max) ; } }
};
}
