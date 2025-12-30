// Generated macro for impl_905 (impl)
macro_rules! Depcrate_unix_linux_systemimpl_905 {
() => {
// Module: crate::unix::linux::system
// Provides: {"impl_905"}
// Dependencies: {}
impl SystemInfo { fn new () -> Self { unsafe { Self { page_size_b : sysconf (_SC_PAGESIZE) as _ , clock_cycle : sysconf (_SC_CLK_TCK) as _ , boot_time : boot_time () , } } } }
};
}
