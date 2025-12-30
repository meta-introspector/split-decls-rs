// Generated macro for impl_594 (impl)
macro_rules! Depcrate_unix_freebsd_cpuimpl_594 {
() => {
// Module: crate::unix::freebsd::cpu
// Provides: {"impl_594"}
// Dependencies: {}
impl CpuInner { pub (crate) fn new (name : String , vendor_id : String , frequency : u64) -> Self { Self { cpu_usage : 0. , name , vendor_id , frequency , } } pub (crate) fn cpu_usage (& self) -> f32 { self . cpu_usage } pub (crate) fn name (& self) -> & str { & self . name } pub (crate) fn frequency (& self) -> u64 { self . frequency } pub (crate) fn vendor_id (& self) -> & str { & self . vendor_id } pub (crate) fn brand (& self) -> & str { "" } }
};
}
