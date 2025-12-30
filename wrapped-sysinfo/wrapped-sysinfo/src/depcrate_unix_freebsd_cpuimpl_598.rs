// Generated macro for impl_598 (impl)
macro_rules! Depcrate_unix_freebsd_cpuimpl_598 {
() => {
// Module: crate::unix::freebsd::cpu
// Provides: {"impl_598"}
// Dependencies: {}
impl < T : Clone > VecSwitcher < T > { pub fn new (v1 : Vec < T >) -> Self { let v2 = v1 . clone () ; Self { v1 , v2 , first : true , } } pub fn get_mut (& mut self) -> & mut [T] { self . first = ! self . first ; if self . first { & mut self . v2 } else { & mut self . v1 } } pub fn get_old (& self) -> & [T] { if self . first { & self . v1 } else { & self . v2 } } pub fn get_new (& self) -> & [T] { if self . first { & self . v2 } else { & self . v1 } } }
};
}
