// Generated macro for impl_515 (impl)
macro_rules! Depcrate_unix_apple_processimpl_515 {
() => {
// Module: crate::unix::apple::process
// Provides: {"impl_515"}
// Dependencies: {}
# [doc (hidden)] impl From < ThreadStatus > for ProcessStatus { fn from (status : ThreadStatus) -> ProcessStatus { match status { ThreadStatus :: Running => ProcessStatus :: Run , ThreadStatus :: Stopped => ProcessStatus :: Stop , ThreadStatus :: Waiting => ProcessStatus :: Sleep , ThreadStatus :: Uninterruptible => ProcessStatus :: Dead , ThreadStatus :: Halted => ProcessStatus :: Parked , ThreadStatus :: Unknown (x) => ProcessStatus :: Unknown (x as _) , } } }
};
}
