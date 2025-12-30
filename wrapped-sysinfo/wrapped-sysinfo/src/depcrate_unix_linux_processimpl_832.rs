// Generated macro for impl_832 (impl)
macro_rules! Depcrate_unix_linux_processimpl_832 {
() => {
// Module: crate::unix::linux::process
// Provides: {"impl_832"}
// Dependencies: {}
# [doc (hidden)] impl From < char > for ProcessStatus { fn from (status : char) -> ProcessStatus { match status { 'R' => ProcessStatus :: Run , 'S' => ProcessStatus :: Sleep , 'I' => ProcessStatus :: Idle , 'D' => ProcessStatus :: UninterruptibleDiskSleep , 'Z' => ProcessStatus :: Zombie , 'T' => ProcessStatus :: Stop , 't' => ProcessStatus :: Tracing , 'X' | 'x' => ProcessStatus :: Dead , 'K' => ProcessStatus :: Wakekill , 'W' => ProcessStatus :: Waking , 'P' => ProcessStatus :: Parked , x => ProcessStatus :: Unknown (x as u32) , } } }
};
}
