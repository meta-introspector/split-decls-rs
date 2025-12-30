// Generated macro for get_proc_list (function)
macro_rules! Depcrate_unix_apple_macos_processget_proc_list {
() => {
// Module: crate::unix::apple::macos::process
// Provides: {"get_proc_list"}
// Dependencies: {}
# [allow (clippy :: uninit_vec)] pub (crate) fn get_proc_list () -> Option < Vec < Pid > > { unsafe { let count = libc :: proc_listallpids (:: std :: ptr :: null_mut () , 0) ; if count < 1 { return None ; } let mut pids : Vec < Pid > = Vec :: with_capacity (count as usize) ; pids . set_len (count as usize) ; let count = count * mem :: size_of :: < Pid > () as i32 ; let x = libc :: proc_listallpids (pids . as_mut_ptr () as * mut c_void , count) ; if x < 1 || x as usize >= pids . len () { None } else { pids . set_len (x as usize) ; Some (pids) } } }
};
}
