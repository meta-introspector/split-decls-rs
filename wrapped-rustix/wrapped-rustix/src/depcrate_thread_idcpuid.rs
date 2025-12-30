// Generated macro for Cpuid (struct)
macro_rules! Depcrate_thread_idCpuid {
() => {
// Module: crate::thread::id
// Provides: {"Cpuid"}
// Dependencies: {}
# [doc = " A Linux CPU ID."] # [cfg (linux_kernel)] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Debug , Hash)] pub struct Cpuid (RawCpuid) ;
};
}
