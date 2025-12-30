// Generated macro for SystemInner (struct)
macro_rules! Depcrate_unix_apple_systemSystemInner {
() => {
// Module: crate::unix::apple::system
// Provides: {"SystemInner"}
// Dependencies: {}
pub (crate) struct SystemInner { process_list : HashMap < Pid , Process > , mem_total : u64 , mem_free : u64 , mem_used : u64 , mem_available : u64 , swap_total : u64 , swap_free : u64 , page_size_b : u64 , port : mach_port_t , # [cfg (all (target_os = "macos" , not (feature = "apple-sandbox")))] clock_info : Option < crate :: sys :: macos :: system :: SystemTimeInfo > , cpus : CpusWrapper , }
};
}
