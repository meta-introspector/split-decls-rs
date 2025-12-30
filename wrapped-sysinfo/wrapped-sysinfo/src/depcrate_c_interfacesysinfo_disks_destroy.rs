// Generated macro for sysinfo_disks_destroy (function)
macro_rules! Depcrate_c_interfacesysinfo_disks_destroy {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_disks_destroy"}
// Dependencies: {}
# [doc = " Equivalent of `Disks::drop()`. Important in C to cleanup memory."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_disks_destroy (disks : CDisks) { assert ! (! disks . is_null ()) ; unsafe { drop (Box :: from_raw (disks as * mut Disks)) ; } }
};
}
