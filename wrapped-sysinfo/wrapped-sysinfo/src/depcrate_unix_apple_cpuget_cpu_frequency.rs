// Generated macro for get_cpu_frequency (function)
macro_rules! Depcrate_unix_apple_cpuget_cpu_frequency {
() => {
// Module: crate::unix::apple::cpu
// Provides: {"get_cpu_frequency"}
// Dependencies: {}
pub (crate) unsafe fn get_cpu_frequency (# [allow (unused_variables)] brand : & str) -> u64 { let mut speed : u64 = 0 ; let mut len = std :: mem :: size_of :: < u64 > () ; if unsafe { libc :: sysctlbyname ("hw.cpufrequency" . as_ptr () as * const _ , & mut speed as * mut _ as _ , & mut len , std :: ptr :: null_mut () , 0 ,) } == 0 { return speed / 1_000_000 ; } # [cfg (any (target_os = "ios" , feature = "apple-sandbox"))] { 0 } # [cfg (not (any (target_os = "ios" , feature = "apple-sandbox")))] unsafe { crate :: sys :: inner :: cpu :: get_cpu_frequency (brand) } }
};
}
