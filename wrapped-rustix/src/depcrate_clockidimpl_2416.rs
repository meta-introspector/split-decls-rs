// Generated macro for impl_2416 (impl)
macro_rules! Depcrate_clockidimpl_2416 {
() => {
// Module: crate::clockid
// Provides: {"impl_2416"}
// Dependencies: {}
# [cfg (not (any (apple , target_os = "wasi")))] impl TryFrom < c :: clockid_t > for ClockId { type Error = io :: Errno ; fn try_from (value : c :: clockid_t) -> Result < Self , Self :: Error > { match value { c :: CLOCK_REALTIME => Ok (ClockId :: Realtime) , c :: CLOCK_MONOTONIC => Ok (ClockId :: Monotonic) , # [cfg (any (freebsdlike , target_os = "openbsd"))] c :: CLOCK_UPTIME => Ok (ClockId :: Uptime) , # [cfg (not (any (solarish , target_os = "horizon" , target_os = "netbsd" , target_os = "redox" , target_os = "vita")))] c :: CLOCK_PROCESS_CPUTIME_ID => Ok (ClockId :: ProcessCPUTime) , # [cfg (not (any (solarish , target_os = "horizon" , target_os = "netbsd" , target_os = "redox" , target_os = "vita")))] c :: CLOCK_THREAD_CPUTIME_ID => Ok (ClockId :: ThreadCPUTime) , # [cfg (any (linux_kernel , target_os = "freebsd"))] c :: CLOCK_REALTIME_COARSE => Ok (ClockId :: RealtimeCoarse) , # [cfg (any (linux_kernel , target_os = "freebsd"))] c :: CLOCK_MONOTONIC_COARSE => Ok (ClockId :: MonotonicCoarse) , # [cfg (linux_kernel)] c :: CLOCK_MONOTONIC_RAW => Ok (ClockId :: MonotonicRaw) , # [cfg (linux_kernel)] c :: CLOCK_REALTIME_ALARM => Ok (ClockId :: RealtimeAlarm) , # [cfg (all (linux_kernel , feature = "linux_4_11"))] c :: CLOCK_TAI => Ok (ClockId :: Tai) , # [cfg (any (linux_kernel , target_os = "fuchsia" , target_os = "openbsd"))] c :: CLOCK_BOOTTIME => Ok (ClockId :: Boottime) , # [cfg (any (linux_kernel , target_os = "fuchsia"))] c :: CLOCK_BOOTTIME_ALARM => Ok (ClockId :: BoottimeAlarm) , _ => Err (io :: Errno :: RANGE) , } } }
};
}
