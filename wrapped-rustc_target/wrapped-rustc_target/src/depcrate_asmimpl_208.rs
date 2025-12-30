// Generated macro for impl_208 (impl)
macro_rules! Depcrate_asmimpl_208 {
() => {
// Module: crate::asm
// Provides: {"impl_208"}
// Dependencies: {}
impl InlineAsmRegOrRegClass { pub fn reg_class (self) -> InlineAsmRegClass { match self { Self :: Reg (r) => r . reg_class () , Self :: RegClass (r) => r , } } }
};
}
