// Generated macro for sysinfo_disks_init (function)
macro_rules! Depcrate_c_interfacesysinfo_disks_init {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_disks_init"}
// Dependencies: {}
# [doc = " Equivalent of [`Disks::new()`][crate::Disks#method.new]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_disks_init () -> CDisks { let disks = Box :: new (Disks :: new ()) ; Box :: into_raw (disks) as CDisks }
};
}
