// Generated macro for VecSwitcher (struct)
macro_rules! Depcrate_unix_freebsd_cpuVecSwitcher {
() => {
// Module: crate::unix::freebsd::cpu
// Provides: {"VecSwitcher"}
// Dependencies: {}
# [doc = " This struct is used to switch between the \"old\" and \"new\" every time you use \"get_mut\"."] # [derive (Debug)] pub (crate) struct VecSwitcher < T > { v1 : Vec < T > , v2 : Vec < T > , first : bool , }
};
}
