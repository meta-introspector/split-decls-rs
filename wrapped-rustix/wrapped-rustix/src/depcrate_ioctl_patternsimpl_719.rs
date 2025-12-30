// Generated macro for impl_719 (impl)
macro_rules! Depcrate_ioctl_patternsimpl_719 {
() => {
// Module: crate::ioctl::patterns
// Provides: {"impl_719"}
// Dependencies: {}
impl < 'a , const OPCODE : Opcode , Value > Updater < 'a , OPCODE , Value > { # [doc = " Create a new pointer updater-style `ioctl` object."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = "  - `OPCODE` must provide a valid opcode."] # [doc = "  - For this opcode, `Value` must be the type that the kernel expects to"] # [doc = "    get."] # [inline] pub unsafe fn new (value : & 'a mut Value) -> Self { Self { value } } }
};
}
