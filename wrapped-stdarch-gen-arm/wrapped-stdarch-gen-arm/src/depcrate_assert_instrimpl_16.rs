// Generated macro for impl_16 (impl)
macro_rules! Depcrate_assert_instrimpl_16 {
() => {
// Module: crate::assert_instr
// Provides: {"impl_16"}
// Dependencies: {}
impl InstructionAssertionMethod { pub (crate) fn build (& mut self , ctx : & Context) -> context :: Result { if let Some (ref mut float) = self . float { float . build (ctx) ? } if let Some (ref mut unsigned) = self . unsigned { unsigned . build (ctx) ? } self . default . build (ctx) } }
};
}
