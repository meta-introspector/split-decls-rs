// Generated macro for Itimerspec (struct)
macro_rules! Depcrate_time_timerfdItimerspec {
() => {
// Module: crate::time::timerfd
// Provides: {"Itimerspec"}
// Dependencies: {}
# [doc = " `struct itimerspec` for use with [`timerfd_gettime`] and"] # [doc = " [`timerfd_settime`]."] # [doc = ""] # [doc = " [`timerfd_gettime`]: crate::time::timerfd_gettime"] # [doc = " [`timerfd_settime`]: crate::time::timerfd_settime"] # [derive (Debug , Clone)] pub struct Itimerspec { # [doc = " Interval between times."] pub it_interval : Timespec , # [doc = " Value of the time."] pub it_value : Timespec , }
};
}
