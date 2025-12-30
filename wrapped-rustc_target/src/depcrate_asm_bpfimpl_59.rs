// Generated macro for impl_59 (impl)
macro_rules! Depcrate_asm_bpfimpl_59 {
() => {
// Module: crate::asm::bpf
// Provides: {"impl_59"}
// Dependencies: {}
impl BpfInlineAsmReg { pub fn emit (self , out : & mut dyn fmt :: Write , _arch : InlineAsmArch , _modifier : Option < char > ,) -> fmt :: Result { out . write_str (self . name ()) } pub fn overlapping_regs (self , mut cb : impl FnMut (BpfInlineAsmReg)) { cb (self) ; macro_rules ! reg_conflicts { ($ ($ r : ident : $ w : ident) ,*) => { match self { $ (Self ::$ r => { cb (Self ::$ w) ; } Self ::$ w => { cb (Self ::$ r) ; }) * } } ; } reg_conflicts ! { r0 : w0 , r1 : w1 , r2 : w2 , r3 : w3 , r4 : w4 , r5 : w5 , r6 : w6 , r7 : w7 , r8 : w8 , r9 : w9 } } }
};
}
