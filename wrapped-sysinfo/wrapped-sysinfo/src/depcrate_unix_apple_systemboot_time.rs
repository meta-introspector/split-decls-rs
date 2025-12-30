// Generated macro for boot_time (function)
macro_rules! Depcrate_unix_apple_systemboot_time {
() => {
// Module: crate::unix::apple::system
// Provides: {"boot_time"}
// Dependencies: {}
fn boot_time () -> u64 { let mut boot_time = timeval { tv_sec : 0 , tv_usec : 0 , } ; let mut len = std :: mem :: size_of :: < timeval > () ; let mut mib : [c_int ; 2] = [libc :: CTL_KERN , libc :: KERN_BOOTTIME] ; unsafe { if sysctl (mib . as_mut_ptr () , mib . len () as _ , & mut boot_time as * mut timeval as * mut _ , & mut len , std :: ptr :: null_mut () , 0 ,) < 0 { 0 } else { boot_time . tv_sec as _ } } }
};
}
