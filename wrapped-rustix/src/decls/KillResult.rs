macro_rules! deps {
    () => {
        Pid!();
    };
}

macro_rules! KillResult {
    () => {
        deps!();
        # [doc = " Reaper status as returned by [`get_reaper_status`]."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct KillResult { # [doc = " The number of processes that were signalled."] pub killed : usize , # [doc = " The pid of the first process that wasn't successfully signalled."] pub first_failed : Option < Pid > , }
    };
}

KillResult!();