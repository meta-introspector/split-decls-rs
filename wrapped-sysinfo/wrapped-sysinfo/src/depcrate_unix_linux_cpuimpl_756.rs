// Generated macro for impl_756 (impl)
macro_rules! Depcrate_unix_linux_cpuimpl_756 {
() => {
// Module: crate::unix::linux::cpu
// Provides: {"impl_756"}
// Dependencies: {}
impl CpuValues { # [doc = " Sets the given argument to the corresponding fields."] pub fn set (& mut self , user : u64 , nice : u64 , system : u64 , idle : u64 , iowait : u64 , irq : u64 , softirq : u64 , steal : u64 , guest : u64 , guest_nice : u64 ,) { self . user = user . saturating_sub (guest) ; self . nice = nice . saturating_sub (guest_nice) ; self . system = system ; self . idle = idle ; self . iowait = iowait ; self . irq = irq ; self . softirq = softirq ; self . steal = steal ; self . guest = guest ; self . guest_nice = guest_nice ; } # [inline] pub fn work_time (& self) -> u64 { self . user . saturating_add (self . nice) } # [inline] pub fn system_time (& self) -> u64 { self . system . saturating_add (self . irq) . saturating_add (self . softirq) } # [inline] pub fn idle_time (& self) -> u64 { self . idle . saturating_add (self . iowait) } # [inline] pub fn virtual_time (& self) -> u64 { self . guest . saturating_add (self . guest_nice) } # [inline] pub fn total_time (& self) -> u64 { self . work_time () . saturating_add (self . system_time ()) . saturating_add (self . idle_time ()) . saturating_add (self . virtual_time ()) . saturating_add (self . steal) } }
};
}
