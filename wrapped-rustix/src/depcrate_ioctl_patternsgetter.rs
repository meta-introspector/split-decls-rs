// Generated macro for Getter (struct)
macro_rules! Depcrate_ioctl_patternsGetter {
() => {
// Module: crate::ioctl::patterns
// Provides: {"Getter"}
// Dependencies: {}
# [doc = " Implements the traditional “getter” pattern for `ioctl`s."] # [doc = ""] # [doc = " Some `ioctl`s just read data into the userspace. As this is a popular"] # [doc = " pattern, this structure implements it."] # [doc = ""] # [doc = " To compute a value for the `OPCODE` argument, see the functions in the"] # [doc = " [`opcode`] module."] # [doc = ""] # [doc = " [`opcode`]: crate::ioctl::opcode"] pub struct Getter < const OPCODE : Opcode , Output > { # [doc = " The output data."] output : mem :: MaybeUninit < Output > , }
};
}
