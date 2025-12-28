macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! NanosleepRelativeResult {
    () => {
        deps!();
        # [doc = " A return type for `nanosleep` and `clock_nanosleep_relative`."] # [derive (Clone)] # [must_use] pub enum NanosleepRelativeResult { # [doc = " The sleep completed normally."] Ok , # [doc = " The sleep was interrupted, the remaining time is returned."] Interrupted (Timespec) , # [doc = " An invalid time value was provided."] Err (io :: Errno) , }
    };
}

NanosleepRelativeResult!();