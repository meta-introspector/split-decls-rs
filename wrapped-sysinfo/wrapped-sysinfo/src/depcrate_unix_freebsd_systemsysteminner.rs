// Generated macro for SystemInner (struct)
macro_rules! Depcrate_unix_freebsd_systemSystemInner {
() => {
// Module: crate::unix::freebsd::system
// Provides: {"SystemInner"}
// Dependencies: {}
pub (crate) struct SystemInner { process_list : HashMap < Pid , Process > , mem_total : u64 , mem_free : u64 , mem_used : u64 , swap_total : u64 , swap_used : u64 , system_info : SystemInfo , cpus : CpusWrapper , }
};
}
