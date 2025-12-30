// Generated macro for impl_1084 (impl)
macro_rules! Depcrate_windows_cpuimpl_1084 {
() => {
// Module: crate::windows::cpu
// Provides: {"impl_1084"}
// Dependencies: {}
impl CpuInner { pub (crate) fn cpu_usage (& self) -> f32 { self . usage . percent } pub (crate) fn name (& self) -> & str { & self . name } pub (crate) fn frequency (& self) -> u64 { self . frequency } pub (crate) fn vendor_id (& self) -> & str { & self . vendor_id } pub (crate) fn brand (& self) -> & str { & self . brand } pub (crate) fn new_with_values (name : String , vendor_id : String , brand : String , frequency : u64 ,) -> Self { Self { name , usage : CpuUsage { percent : 0f32 , key_used : None , } , vendor_id , brand , frequency , } } pub (crate) fn set_cpu_usage (& mut self , value : f32) { self . usage . set_cpu_usage (value) ; } pub (crate) fn set_frequency (& mut self , value : u64) { self . frequency = value ; } }
};
}
