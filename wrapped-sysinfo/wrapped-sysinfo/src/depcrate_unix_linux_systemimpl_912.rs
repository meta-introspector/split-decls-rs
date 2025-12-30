// Generated macro for impl_912 (impl)
macro_rules! Depcrate_unix_linux_systemimpl_912 {
() => {
// Module: crate::unix::linux::system
// Provides: {"impl_912"}
// Dependencies: {}
impl crate :: CGroupLimits { fn new (sys : & SystemInner) -> Option < Self > { assert ! (sys . mem_total != 0 , "You need to call System::refresh_memory before trying to get cgroup limits!" ,) ; if let (Some (mem_cur) , Some (mem_max) , Some (mem_rss)) = (read_u64 ("/sys/fs/cgroup/memory.current") , read_u64 ("/sys/fs/cgroup/memory.max") . or (Some (u64 :: MAX)) , read_table_key ("/sys/fs/cgroup/memory.stat" , "anon" , ' ') ,) { let mut limits = Self { total_memory : sys . mem_total , free_memory : sys . mem_free , free_swap : sys . swap_free , rss : mem_rss , } ; limits . total_memory = min (mem_max , sys . mem_total) ; limits . free_memory = limits . total_memory . saturating_sub (mem_cur) ; if let Some (swap_cur) = read_u64 ("/sys/fs/cgroup/memory.swap.current") { limits . free_swap = sys . swap_total . saturating_sub (swap_cur) ; } Some (limits) } else if let (Some (mem_cur) , Some (mem_max) , Some (mem_rss)) = (read_u64 ("/sys/fs/cgroup/memory/memory.usage_in_bytes") , read_u64 ("/sys/fs/cgroup/memory/memory.limit_in_bytes") , read_table_key ("/sys/fs/cgroup/memory/memory.stat" , "total_rss" , ' ') ,) { let mut limits = Self { total_memory : sys . mem_total , free_memory : sys . mem_free , free_swap : sys . swap_free , rss : mem_rss , } ; limits . total_memory = min (mem_max , sys . mem_total) ; limits . free_memory = limits . total_memory . saturating_sub (mem_cur) ; Some (limits) } else { None } } }
};
}
