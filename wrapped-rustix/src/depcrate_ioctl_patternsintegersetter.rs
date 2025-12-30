// Generated macro for IntegerSetter (struct)
macro_rules! Depcrate_ioctl_patternsIntegerSetter {
() => {
// Module: crate::ioctl::patterns
// Provides: {"IntegerSetter"}
// Dependencies: {}
# [doc = " Implements an `ioctl` that passes an integer into the `ioctl`."] # [doc = ""] # [doc = " To compute a value for the `OPCODE` argument, see the functions in the"] # [doc = " [`opcode`] module."] # [doc = ""] # [doc = " [`opcode`]: crate::ioctl::opcode"] pub struct IntegerSetter < const OPCODE : Opcode > { # [doc = " The value to pass in."] # [doc = ""] # [doc = " For strict provenance preservation, this is a pointer."] value : * mut c :: c_void , }
};
}
