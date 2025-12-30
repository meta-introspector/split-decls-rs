// Generated macro for CGroupLimits (struct)
macro_rules! Depcrate_common_systemCGroupLimits {
() => {
// Module: crate::common::system
// Provides: {"CGroupLimits"}
// Dependencies: {}
# [doc = " Contains memory limits for the current process."] # [derive (Default , Debug , Clone)] pub struct CGroupLimits { # [doc = " Total memory (in bytes) for the current cgroup."] pub total_memory : u64 , # [doc = " Free memory (in bytes) for the current cgroup."] pub free_memory : u64 , # [doc = " Free swap (in bytes) for the current cgroup."] pub free_swap : u64 , # [doc = " Resident Set Size (RSS) (in bytes) for the current cgroup."] pub rss : u64 , }
};
}
