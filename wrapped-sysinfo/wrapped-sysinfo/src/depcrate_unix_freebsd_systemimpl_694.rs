// Generated macro for impl_694 (impl)
macro_rules! Depcrate_unix_freebsd_systemimpl_694 {
() => {
// Module: crate::unix::freebsd::system
// Provides: {"impl_694"}
// Dependencies: {}
impl Zfs { fn new () -> Self { let mut zfs = Self { enabled : false , mib_arcstats_size : Default :: default () , } ; unsafe { init_mib (b"kstat.zfs.misc.arcstats.size\0" , & mut zfs . mib_arcstats_size ,) ; let mut arc_size : u64 = 0 ; if get_sys_value (& zfs . mib_arcstats_size , & mut arc_size) { zfs . enabled = arc_size != 0 ; } } zfs } fn arc_size (& self) -> Option < u64 > { if self . enabled { let mut arc_size : u64 = 0 ; unsafe { get_sys_value (& self . mib_arcstats_size , & mut arc_size) ; Some (arc_size) } } else { None } } }
};
}
