// Generated macro for impl_1955 (impl)
macro_rules! Depcrate_thread_schedimpl_1955 {
() => {
// Module: crate::thread::sched
// Provides: {"impl_1955"}
// Dependencies: {}
impl hash :: Hash for CpuSet { fn hash < H : hash :: Hasher > (& self , state : & mut H) { for i in 0 .. Self :: MAX_CPU { self . is_set (i) . hash (state) ; } } }
};
}
