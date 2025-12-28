macro_rules! macro_926 {
    () => {
        bitflags ! { # [doc = " `PR_UNALIGN_*` flags for use with [`unaligned_access_control`] and"] # [doc = " [`set_unaligned_access_control`]."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct UnalignedAccessControl : u32 { # [doc = " Silently fix up unaligned user accesses."] # [doc (alias = "NOPRINT")] # [doc (alias = "PR_UNALIGN_NOPRINT")] const NO_PRINT = 1 ; # [doc = " Generate a [`Signal::Bus`] signal on unaligned user access."] # [doc (alias = "PR_UNALIGN_SIGBUS")] const SIGBUS = 2 ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
    };
}

macro_926!();