// Generated macro for sysinfo_disks_refresh (function)
macro_rules! Depcrate_c_interfacesysinfo_disks_refresh {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_disks_refresh"}
// Dependencies: {}
# [doc = " Equivalent of [`Disks::refresh()`][crate::Disks#method.refresh]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_disks_refresh (disks : CDisks) { assert ! (! disks . is_null ()) ; unsafe { let mut disks : Box < Disks > = Box :: from_raw (disks as * mut Disks) ; { let disks : & mut Disks = disks . borrow_mut () ; disks . refresh (true) ; } let _ = Box :: into_raw (disks) ; } }
};
}
