// Generated macro for discard_inherited_jobserver (function)
macro_rules! Depcrate_jobserverdiscard_inherited_jobserver {
() => {
// Module: crate::jobserver
// Provides: {"discard_inherited_jobserver"}
// Dependencies: {}
# [cfg (not (windows))] pub unsafe fn discard_inherited_jobserver () { if let Some (value) = ["CARGO_MAKEFLAGS" , "MAKEFLAGS" , "MFLAGS"] . into_iter () . find_map (| env | std :: env :: var (env) . ok ()) { if let Some (auth) = value . rsplit (' ') . find_map (| arg | { arg . strip_prefix ("--jobserver-auth=") . or_else (| | arg . strip_prefix ("--jobserver-fds=")) }) { if ! auth . starts_with ("fifo:") { let mut parts = auth . splitn (2 , ',') ; let read = parts . next () . unwrap () ; let write = match parts . next () { Some (w) => w , None => return , } ; let read = read . parse () . unwrap () ; let write = write . parse () . unwrap () ; if read < 0 || write < 0 { return ; } unsafe { if libc :: fcntl (read , libc :: F_GETFD) == - 1 { return ; } if libc :: fcntl (write , libc :: F_GETFD) == - 1 { return ; } libc :: close (read) ; libc :: close (write) ; } } } } }
};
}
