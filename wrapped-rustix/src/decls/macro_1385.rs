macro_rules! macro_1385 {
    () => {
        bitflags ! { # [doc = " `CLONE_*` for use with [`unshare`]."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct UnshareFlags : u32 { # [doc = " `CLONE_FILES`"] const FILES = CLONE_FILES ; # [doc = " `CLONE_FS`"] const FS = CLONE_FS ; # [doc = " `CLONE_NEWCGROUP`"] const NEWCGROUP = CLONE_NEWCGROUP ; # [doc = " `CLONE_NEWIPC`"] const NEWIPC = CLONE_NEWIPC ; # [doc = " `CLONE_NEWNET`"] const NEWNET = CLONE_NEWNET ; # [doc = " `CLONE_NEWNS`"] const NEWNS = CLONE_NEWNS ; # [doc = " `CLONE_NEWPID`"] const NEWPID = CLONE_NEWPID ; # [doc = " `CLONE_NEWTIME`"] const NEWTIME = CLONE_NEWTIME ; # [doc = " `CLONE_NEWUSER`"] const NEWUSER = CLONE_NEWUSER ; # [doc = " `CLONE_NEWUTS`"] const NEWUTS = CLONE_NEWUTS ; # [doc = " `CLONE_SYSVSEM`"] const SYSVSEM = CLONE_SYSVSEM ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_1385!()