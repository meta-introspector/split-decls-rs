// Generated macro for impl_1957 (impl)
macro_rules! Depcrate_thread_schedimpl_1957 {
() => {
// Module: crate::thread::sched
// Provides: {"impl_1957"}
// Dependencies: {}
impl PartialEq for CpuSet { fn eq (& self , other : & Self) -> bool { backend :: thread :: cpu_set :: CPU_EQUAL (& self . cpu_set , & other . cpu_set) } }
};
}
