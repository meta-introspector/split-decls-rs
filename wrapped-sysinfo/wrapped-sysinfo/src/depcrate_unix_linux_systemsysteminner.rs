// Generated macro for SystemInner (struct)
macro_rules! Depcrate_unix_linux_systemSystemInner {
() => {
// Module: crate::unix::linux::system
// Provides: {"SystemInner"}
// Dependencies: {}
pub (crate) struct SystemInner { process_list : HashMap < Pid , Process > , mem_total : u64 , mem_free : u64 , mem_available : u64 , mem_buffers : u64 , mem_page_cache : u64 , mem_shmem : u64 , mem_slab_reclaimable : u64 , swap_total : u64 , swap_free : u64 , info : SystemInfo , cpus : CpusWrapper , }
};
}
