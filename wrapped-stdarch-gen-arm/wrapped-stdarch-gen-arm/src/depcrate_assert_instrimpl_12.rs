// Generated macro for impl_12 (impl)
macro_rules! Depcrate_assert_instrimpl_12 {
() => {
// Module: crate::assert_instr
// Provides: {"impl_12"}
// Dependencies: {}
impl InstructionAssertionMethodForBitsize { fn build (& mut self , ctx : & Context) -> context :: Result { if let Some (ref mut byte) = self . byte { byte . build (ctx) ? } if let Some (ref mut halfword) = self . halfword { halfword . build (ctx) ? } if let Some (ref mut word) = self . word { word . build (ctx) ? } if let Some (ref mut doubleword) = self . doubleword { doubleword . build (ctx) ? } self . default . build (ctx) } }
};
}
