macro_rules! deps {
    () => {
        Statx!();
    };
}

macro_rules! macro_1569 {
    () => {
        deps!();
        bitflags ! { # [doc = " `STATX_ATTR_*` flags for use with [`Statx`]."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct StatxAttributes : u64 { # [doc = " `STATX_ATTR_COMPRESSED`"] const COMPRESSED = c :: STATX_ATTR_COMPRESSED as u64 ; # [doc = " `STATX_ATTR_IMMUTABLE`"] const IMMUTABLE = c :: STATX_ATTR_IMMUTABLE as u64 ; # [doc = " `STATX_ATTR_APPEND`"] const APPEND = c :: STATX_ATTR_APPEND as u64 ; # [doc = " `STATX_ATTR_NODUMP`"] const NODUMP = c :: STATX_ATTR_NODUMP as u64 ; # [doc = " `STATX_ATTR_ENCRYPTED`"] const ENCRYPTED = c :: STATX_ATTR_ENCRYPTED as u64 ; # [doc = " `STATX_ATTR_AUTOMOUNT`"] const AUTOMOUNT = c :: STATX_ATTR_AUTOMOUNT as u64 ; # [doc = " `STATX_ATTR_MOUNT_ROOT`"] const MOUNT_ROOT = c :: STATX_ATTR_MOUNT_ROOT as u64 ; # [doc = " `STATX_ATTR_VERITY`"] const VERITY = c :: STATX_ATTR_VERITY as u64 ; # [doc = " `STATX_ATTR_DAX`"] const DAX = c :: STATX_ATTR_DAX as u64 ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_1569!();