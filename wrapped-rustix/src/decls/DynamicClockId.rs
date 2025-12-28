macro_rules! deps {
    () => {
        ClockId!();
    };
}

macro_rules! DynamicClockId {
    () => {
        deps!();
        # [doc = " `CLOCK_*` constants for use with [`clock_gettime_dynamic`]."] # [doc = ""] # [doc = " These constants may be unsupported at runtime, depending on the OS version,"] # [doc = " and `clock_gettime_dynamic` may fail with `INVAL`. See [`ClockId`] for"] # [doc = " clocks which are always supported at runtime."] # [doc = ""] # [doc = " [`clock_gettime_dynamic`]: crate::time::clock_gettime_dynamic"] # [cfg (not (target_os = "wasi"))] # [derive (Debug , Copy , Clone)] # [non_exhaustive] pub enum DynamicClockId < 'a > { # [doc = " `ClockId` values that are always supported at runtime."] Known (ClockId) , # [doc = " Linux dynamic clocks."] Dynamic (BorrowedFd < 'a >) , # [doc = " `CLOCK_REALTIME_ALARM`"] # [cfg (any (linux_kernel , target_os = "fuchsia"))] # [doc (alias = "CLOCK_REALTIME_ALARM")] RealtimeAlarm , # [doc = " `CLOCK_TAI`, available on Linux ≥ 3.10"] # [cfg (linux_kernel)] # [doc (alias = "CLOCK_TAI")] Tai , # [doc = " `CLOCK_BOOTTIME`"] # [cfg (any (linux_kernel , target_os = "freebsd" , target_os = "fuchsia" , target_os = "openbsd"))] # [doc (alias = "CLOCK_BOOTTIME")] Boottime , # [doc = " `CLOCK_BOOTTIME_ALARM`"] # [cfg (any (linux_kernel , target_os = "fuchsia"))] # [doc (alias = "CLOCK_BOOTTIME_ALARM")] BoottimeAlarm , }
    };
}

DynamicClockId!();