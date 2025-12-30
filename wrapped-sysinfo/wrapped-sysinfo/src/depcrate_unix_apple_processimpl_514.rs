// Generated macro for impl_514 (impl)
macro_rules! Depcrate_unix_apple_processimpl_514 {
() => {
// Module: crate::unix::apple::process
// Provides: {"impl_514"}
// Dependencies: {}
# [doc (hidden)] impl From < u32 > for ProcessStatus { fn from (status : u32) -> ProcessStatus { match status { libc :: SIDL => ProcessStatus :: Idle , libc :: SRUN => ProcessStatus :: Run , libc :: SSLEEP => ProcessStatus :: Sleep , libc :: SSTOP => ProcessStatus :: Stop , libc :: SZOMB => ProcessStatus :: Zombie , x => ProcessStatus :: Unknown (x) , } } }
};
}
