// Generated macro for Sysvar (trait)
macro_rules! DepcrateSysvar {
() => {
// Module: crate
// Provides: {"Sysvar"}
// Dependencies: {}
# [doc = " Interface for loading a sysvar."] pub trait Sysvar : Default + Sized { # [doc = " Load the sysvar directly from the runtime."] # [doc = ""] # [doc = " This is the preferred way to load a sysvar. Calling this method does not"] # [doc = " incur any deserialization overhead, and does not require the sysvar"] # [doc = " account to be passed to the program."] # [doc = ""] # [doc = " Not all sysvars support this method. If not, it returns"] # [doc = " [`ProgramError::UnsupportedSysvar`]."] fn get () -> Result < Self , ProgramError > { Err (ProgramError :: UnsupportedSysvar) } }
};
}
