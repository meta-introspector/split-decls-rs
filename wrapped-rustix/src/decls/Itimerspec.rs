macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! Itimerspec {
    () => {
        deps!();
        # [doc = " `struct itimerspec` for use with [`timerfd_gettime`] and"] # [doc = " [`timerfd_settime`]."] # [doc = ""] # [doc = " [`timerfd_gettime`]: crate::time::timerfd_gettime"] # [doc = " [`timerfd_settime`]: crate::time::timerfd_settime"] # [derive (Debug , Clone)] pub struct Itimerspec { # [doc = " Interval between times."] pub it_interval : Timespec , # [doc = " Value of the time."] pub it_value : Timespec , }
    };
}

Itimerspec!();