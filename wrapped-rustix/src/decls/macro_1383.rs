macro_rules! macro_1383 {
    () => {
        bitflags ! { # [doc = " Thread name space type."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct ThreadNameSpaceType : u32 { # [doc = " Time name space."] const TIME = CLONE_NEWTIME ; # [doc = " Mount name space."] const MOUNT = CLONE_NEWNS ; # [doc = " Control group (CGroup) name space."] const CONTROL_GROUP = CLONE_NEWCGROUP ; # [doc = " `Host name` and `NIS domain name` (UTS) name space."] const HOST_NAME_AND_NIS_DOMAIN_NAME = CLONE_NEWUTS ; # [doc = " Inter-process communication (IPC) name space."] const INTER_PROCESS_COMMUNICATION = CLONE_NEWIPC ; # [doc = " User name space."] const USER = CLONE_NEWUSER ; # [doc = " Process ID name space."] const PROCESS_ID = CLONE_NEWPID ; # [doc = " Network name space."] const NETWORK = CLONE_NEWNET ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_1383!();