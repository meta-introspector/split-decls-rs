// Generated macro for impl_51 (impl)
macro_rules! Depcrate_asm_avrimpl_51 {
() => {
// Module: crate::asm::avr
// Provides: {"impl_51"}
// Dependencies: {}
impl AvrInlineAsmReg { pub fn emit (self , out : & mut dyn fmt :: Write , _arch : InlineAsmArch , modifier : Option < char > ,) -> fmt :: Result { let name = emit_pairs ! { self modifier , Z "Z" "ZH" "ZL" , X "X" "XH" "XL" , r25r24 "r25:r24" "r25" "r24" , r23r22 "r23:r22" "r23" "r22" , r21r20 "r21:r20" "r21" "r20" , r19r18 "r19:r18" "r19" "r18" , r17r16 "r17:r16" "r17" "r16" , r15r14 "r15:r14" "r15" "r14" , r13r12 "r13:r12" "r13" "r12" , r11r10 "r11:r10" "r11" "r10" , r9r8 "r9:r8" "r9" "r8" , r7r6 "r7:r6" "r7" "r6" , r5r4 "r5:r4" "r5" "r4" , r3r2 "r3:r2" "r3" "r2" , } ; out . write_str (name) } pub fn overlapping_regs (self , mut cb : impl FnMut (AvrInlineAsmReg)) { cb (self) ; macro_rules ! reg_conflicts { ($ ($ pair : ident : $ hi : ident $ lo : ident ,) *) => { match self { $ (Self ::$ pair => { cb (Self ::$ hi) ; cb (Self ::$ lo) ; } Self ::$ hi => { cb (Self ::$ pair) ; } Self ::$ lo => { cb (Self ::$ pair) ; }) * } } ; } reg_conflicts ! { Z : r31 r30 , X : r27 r26 , r25r24 : r25 r24 , r23r22 : r23 r22 , r21r20 : r21 r20 , r19r18 : r19 r18 , r17r16 : r17 r16 , r15r14 : r15 r14 , r13r12 : r13 r12 , r11r10 : r11 r10 , r9r8 : r9 r8 , r7r6 : r7 r6 , r5r4 : r5 r4 , r3r2 : r3 r2 , } } }
};
}
