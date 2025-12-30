// Generated macro for impl_75 (impl)
macro_rules! Depcrate_asm_hexagonimpl_75 {
() => {
// Module: crate::asm::hexagon
// Provides: {"impl_75"}
// Dependencies: {}
impl HexagonInlineAsmReg { pub fn emit (self , out : & mut dyn fmt :: Write , _arch : InlineAsmArch , _modifier : Option < char > ,) -> fmt :: Result { out . write_str (self . name ()) } pub fn overlapping_regs (self , mut _cb : impl FnMut (HexagonInlineAsmReg)) { } }
};
}
