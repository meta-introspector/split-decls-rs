// Generated macro for impl_658 (impl)
macro_rules! Depcrate_unix_freebsd_processimpl_658 {
() => {
// Module: crate::unix::freebsd::process
// Provides: {"impl_658"}
// Dependencies: {}
# [doc (hidden)] impl From < libc :: c_char > for ProcessStatus { fn from (status : libc :: c_char) -> ProcessStatus { match status { libc :: SIDL => ProcessStatus :: Idle , libc :: SRUN => ProcessStatus :: Run , libc :: SSLEEP => ProcessStatus :: Sleep , libc :: SSTOP => ProcessStatus :: Stop , libc :: SZOMB => ProcessStatus :: Zombie , libc :: SWAIT => ProcessStatus :: Dead , libc :: SLOCK => ProcessStatus :: LockBlocked , x => ProcessStatus :: Unknown (x as _) , } } }
};
}
