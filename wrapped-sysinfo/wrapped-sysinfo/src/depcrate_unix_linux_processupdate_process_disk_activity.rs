// Generated macro for update_process_disk_activity (function)
macro_rules! Depcrate_unix_linux_processupdate_process_disk_activity {
() => {
// Module: crate::unix::linux::process
// Provides: {"update_process_disk_activity"}
// Dependencies: {}
pub (crate) fn update_process_disk_activity (p : & mut ProcessInner , path : & mut PathHandler) { let data = match get_all_utf8_data (path . replace_and_join ("io") , 16_384) { Ok (d) => d , Err (_) => return , } ; let mut done = 0 ; for line in data . split ('\n') { let mut parts = line . split (": ") ; match parts . next () { Some ("read_bytes") => { p . old_read_bytes = p . read_bytes ; p . read_bytes = parts . next () . and_then (| x | x . parse :: < u64 > () . ok ()) . unwrap_or (p . old_read_bytes) ; } Some ("write_bytes") => { p . old_written_bytes = p . written_bytes ; p . written_bytes = parts . next () . and_then (| x | x . parse :: < u64 > () . ok ()) . unwrap_or (p . old_written_bytes) ; } _ => continue , } done += 1 ; if done > 1 { break ; } } }
};
}
