// Generated macro for Updater (struct)
macro_rules! Depcrate_ioctl_patternsUpdater {
() => {
// Module: crate::ioctl::patterns
// Provides: {"Updater"}
// Dependencies: {}
# [doc = " Implements an “updater” pattern for `ioctl`s."] # [doc = ""] # [doc = " The ioctl takes a reference to a struct that it reads its input from,"] # [doc = " then writes output to the same struct."] # [doc = ""] # [doc = " To compute a value for the `OPCODE` argument, see the functions in the"] # [doc = " [`opcode`] module."] # [doc = ""] # [doc = " [`opcode`]: crate::ioctl::opcode"] pub struct Updater < 'a , const OPCODE : Opcode , Value > { # [doc = " Reference to input/output data."] value : & 'a mut Value , }
};
}
