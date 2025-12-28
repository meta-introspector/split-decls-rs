macro_rules! deps {
    () => {
        DynamicClockId!();
    };
}

macro_rules! ClockId {
    () => {
        deps!();
        # [doc = " `CLOCK_*` constants for use with [`clock_gettime`]."] # [doc = ""] # [doc = " These constants are always supported at runtime, so `clock_gettime` never"] # [doc = " has to fail with `INVAL` due to an unsupported clock. See"] # [doc = " [`DynamicClockId`] for a greater set of clocks, with the caveat that not"] # [doc = " all of them are always supported."] # [doc = ""] # [doc = " [`clock_gettime`]: crate::time::clock_gettime"] # [cfg (apple)] # [derive (Debug , Copy , Clone , Eq , PartialEq , Hash)] # [repr (u32)] # [non_exhaustive] pub enum ClockId { # [doc = " `CLOCK_REALTIME`"] # [doc (alias = "CLOCK_REALTIME")] Realtime = c :: CLOCK_REALTIME , # [doc = " `CLOCK_MONOTONIC`"] # [doc (alias = "CLOCK_MONOTONIC")] Monotonic = c :: CLOCK_MONOTONIC , # [doc = " `CLOCK_PROCESS_CPUTIME_ID`"] # [doc (alias = "CLOCK_PROCESS_CPUTIME_ID")] ProcessCPUTime = c :: CLOCK_PROCESS_CPUTIME_ID , # [doc = " `CLOCK_THREAD_CPUTIME_ID`"] # [doc (alias = "CLOCK_THREAD_CPUTIME_ID")] ThreadCPUTime = c :: CLOCK_THREAD_CPUTIME_ID , }
    };
}

ClockId!()