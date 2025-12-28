macro_rules! macro_1083 {
    () => {
        bitflags ! { # [doc = " `REAPER_KILL_*`"] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] struct KillFlags : c_uint { const CHILDREN = 1 ; const SUBTREE = 2 ; } }
    };
}

macro_1083!()