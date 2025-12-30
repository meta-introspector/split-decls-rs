// Generated macro for impl_145 (impl)
macro_rules! Depcrate_asm_s390ximpl_145 {
() => {
// Module: crate::asm::s390x
// Provides: {"impl_145"}
// Dependencies: {}
impl S390xInlineAsmReg { pub fn emit (self , out : & mut dyn fmt :: Write , _arch : InlineAsmArch , _modifier : Option < char > ,) -> fmt :: Result { write ! (out , "%{}" , self . name ()) } pub fn overlapping_regs (self , mut cb : impl FnMut (S390xInlineAsmReg)) { macro_rules ! reg_conflicts { ($ ($ full : ident : $ ($ field : ident) *) ,*;) => { match self { $ (Self ::$ full => { cb (Self ::$ full) ; $ (cb (Self ::$ field) ;) * } $ (Self ::$ field) |* => { cb (Self ::$ full) ; cb (self) ; }) * r => cb (r) , } } ; } reg_conflicts ! { v0 : f0 , v1 : f1 , v2 : f2 , v3 : f3 , v4 : f4 , v5 : f5 , v6 : f6 , v7 : f7 , v8 : f8 , v9 : f9 , v10 : f10 , v11 : f11 , v12 : f12 , v13 : f13 , v14 : f14 , v15 : f15 ; } } }
};
}
