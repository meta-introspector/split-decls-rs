macro_rules! macro_931 {
    () => {
        bitflags ! { # [doc = " `PR_FPEMU_*` flags for use with [`floating_point_emulation_control`]"] # [doc = " and [`set_floating_point_emulation_control`]."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct FloatingPointEmulationControl : u32 { # [doc = " Silently emulate floating point operations accesses."] # [doc (alias = "PR_UNALIGN_NOPRINT")] const NO_PRINT = 1 ; # [doc = " Don't emulate floating point operations, send a [`Signal::Fpe`]"] # [doc = " signal instead."] # [doc (alias = "PR_UNALIGN_SIGFPE")] const SIGFPE = 2 ; } }
    };
}

macro_931!()