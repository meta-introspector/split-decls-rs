// Generated macro for SystemInner (struct)
macro_rules! Depcrate_windows_systemSystemInner {
() => {
// Module: crate::windows::system
// Provides: {"SystemInner"}
// Dependencies: {}
pub (crate) struct SystemInner { process_list : HashMap < Pid , Process > , mem_total : u64 , mem_available : u64 , swap_total : u64 , swap_used : u64 , cpus : CpusWrapper , query : Option < Query > , }
};
}
