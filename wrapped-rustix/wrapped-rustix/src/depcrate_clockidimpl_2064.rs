// Generated macro for impl_2064 (impl)
macro_rules! Depcrate_clockidimpl_2064 {
() => {
// Module: crate::clockid
// Provides: {"impl_2064"}
// Dependencies: {}
# [cfg (apple)] impl TryFrom < c :: clockid_t > for ClockId { type Error = io :: Errno ; fn try_from (value : c :: clockid_t) -> Result < Self , Self :: Error > { match value { c :: CLOCK_REALTIME => Ok (ClockId :: Realtime) , c :: CLOCK_MONOTONIC => Ok (ClockId :: Monotonic) , c :: CLOCK_PROCESS_CPUTIME_ID => Ok (ClockId :: ProcessCPUTime) , c :: CLOCK_THREAD_CPUTIME_ID => Ok (ClockId :: ThreadCPUTime) , _ => Err (io :: Errno :: RANGE) , } } }
};
}
