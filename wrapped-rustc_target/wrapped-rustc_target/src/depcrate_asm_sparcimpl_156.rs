// Generated macro for impl_156 (impl)
macro_rules! Depcrate_asm_sparcimpl_156 {
() => {
// Module: crate::asm::sparc
// Provides: {"impl_156"}
// Dependencies: {}
impl SparcInlineAsmReg { pub fn emit (self , out : & mut dyn fmt :: Write , _arch : InlineAsmArch , _modifier : Option < char > ,) -> fmt :: Result { write ! (out , "%{}" , self . name ()) } }
};
}
