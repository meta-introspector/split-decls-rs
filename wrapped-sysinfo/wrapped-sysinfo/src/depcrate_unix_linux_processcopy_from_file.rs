// Generated macro for copy_from_file (function)
macro_rules! Depcrate_unix_linux_processcopy_from_file {
() => {
// Module: crate::unix::linux::process
// Provides: {"copy_from_file"}
// Dependencies: {}
fn copy_from_file (entry : & Path) -> Vec < OsString > { match File :: open (entry) { Ok (mut f) => { let mut data = Vec :: with_capacity (16_384) ; if let Err (_e) = f . read_to_end (& mut data) { sysinfo_debug ! ("Failed to read file in `copy_from_file`: {:?}" , _e) ; Vec :: new () } else { split_content (& data) } } Err (_e) => { sysinfo_debug ! ("Failed to open file in `copy_from_file`: {:?}" , _e) ; Vec :: new () } } }
};
}
