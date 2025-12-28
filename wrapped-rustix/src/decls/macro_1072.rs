macro_rules! macro_1072 {
    () => {
        bitflags ! { # [doc = " `REAPER_STATUS_*`"] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct ReaperStatusFlags : c_uint { # [doc = " The process has acquired reaper status."] const OWNED = 1 ; # [doc = " The process is the root of the reaper tree ([`Pid::INIT`])."] const REALINIT = 2 ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_1072!()