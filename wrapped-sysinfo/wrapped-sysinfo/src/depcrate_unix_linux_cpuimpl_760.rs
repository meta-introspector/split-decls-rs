// Generated macro for impl_760 (impl)
macro_rules! Depcrate_unix_linux_cpuimpl_760 {
() => {
// Module: crate::unix::linux::cpu
// Provides: {"impl_760"}
// Dependencies: {}
impl CpuInner { pub (crate) fn new_with_values (name : & str , user : u64 , nice : u64 , system : u64 , idle : u64 , iowait : u64 , irq : u64 , softirq : u64 , steal : u64 , guest : u64 , guest_nice : u64 , frequency : u64 , vendor_id : String , brand : String ,) -> Self { Self { usage : CpuUsage :: new_with_values (user , nice , system , idle , iowait , irq , softirq , steal , guest , guest_nice ,) , name : name . to_owned () , frequency , vendor_id , brand , } } pub (crate) fn set (& mut self , user : u64 , nice : u64 , system : u64 , idle : u64 , iowait : u64 , irq : u64 , softirq : u64 , steal : u64 , guest : u64 , guest_nice : u64 ,) { self . usage . set (user , nice , system , idle , iowait , irq , softirq , steal , guest , guest_nice ,) ; } pub (crate) fn cpu_usage (& self) -> f32 { self . usage . percent } pub (crate) fn name (& self) -> & str { & self . name } # [doc = " Returns the CPU frequency in MHz."] pub (crate) fn frequency (& self) -> u64 { self . frequency } pub (crate) fn vendor_id (& self) -> & str { & self . vendor_id } pub (crate) fn brand (& self) -> & str { & self . brand } }
};
}
