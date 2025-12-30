// Generated macro for impl_341 (impl)
macro_rules! Depcrate_unix_apple_cpuimpl_341 {
() => {
// Module: crate::unix::apple::cpu
// Provides: {"impl_341"}
// Dependencies: {}
impl CpuInner { pub (crate) fn new (name : String , cpu_data : Arc < CpuData > , frequency : u64 , vendor_id : String , brand : String ,) -> Self { Self { name , usage : CpuUsage { percent : 0. , data : cpu_data , frequency , } , vendor_id , brand , } } pub (crate) fn set_cpu_usage (& mut self , cpu_usage : f32) { self . usage . set_cpu_usage (cpu_usage) ; } pub (crate) fn update (& mut self , cpu_usage : f32 , cpu_data : Arc < CpuData >) { self . usage . percent = cpu_usage ; self . usage . data = cpu_data ; } pub (crate) fn data (& self) -> Arc < CpuData > { Arc :: clone (& self . usage . data) } pub (crate) fn set_frequency (& mut self , frequency : u64) { self . usage . frequency = frequency ; } pub (crate) fn cpu_usage (& self) -> f32 { self . usage . percent () } pub (crate) fn name (& self) -> & str { & self . name } pub (crate) fn frequency (& self) -> u64 { self . usage . frequency } pub (crate) fn vendor_id (& self) -> & str { & self . vendor_id } pub (crate) fn brand (& self) -> & str { & self . brand } }
};
}
