// Generated macro for wait_process (function)
macro_rules! Depcrate_unix_utilswait_process {
() => {
// Module: crate::unix::utils
// Provides: {"wait_process"}
// Dependencies: {}
# [cfg (all (feature = "system" , not (any (target_os = "ios" , all (target_os = "macos" , feature = "apple-sandbox" ,)))))] pub (crate) fn wait_process (pid : crate :: Pid) -> Option < std :: process :: ExitStatus > { use std :: os :: unix :: process :: ExitStatusExt ; let mut status = 0 ; unsafe { if retry_eintr ! (libc :: waitpid (pid . 0 , & mut status , 0)) < 0 { let duration = std :: time :: Duration :: from_millis (10) ; while libc :: kill (pid . 0 , 0) == 0 { std :: thread :: sleep (duration) ; } } Some (std :: process :: ExitStatus :: from_raw (status)) } }
};
}
