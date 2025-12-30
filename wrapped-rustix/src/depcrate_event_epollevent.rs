// Generated macro for Event (struct)
macro_rules! Depcrate_event_epollEvent {
() => {
// Module: crate::event::epoll
// Provides: {"Event"}
// Dependencies: {}
# [doc = " A record of an event that occurred."] # [repr (C)] # [cfg_attr (all (not (libc) , target_arch = "x86_64") , repr (packed))] # [cfg_attr (all (libc , linux_kernel , any (all (target_arch = "x86" , not (target_env = "musl") , not (target_os = "android") ,) , target_arch = "x86_64" ,)) , repr (packed))] # [cfg_attr (all (solarish , any (target_arch = "x86" , target_arch = "x86_64")) , repr (packed (4)))] # [derive (Copy , Clone , Eq , PartialEq , Hash)] pub struct Event { # [doc = " Which specific event(s) occurred."] pub flags : EventFlags , # [doc = " User data."] pub data : EventData , # [cfg (all (libc , target_os = "redox"))] _pad : u64 , }
};
}
