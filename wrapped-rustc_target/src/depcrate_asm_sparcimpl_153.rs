// Generated macro for impl_153 (impl)
macro_rules! Depcrate_asm_sparcimpl_153 {
() => {
// Module: crate::asm::sparc
// Provides: {"impl_153"}
// Dependencies: {}
impl SparcInlineAsmRegClass { pub fn valid_modifiers (self , _arch : super :: InlineAsmArch) -> & 'static [char] { & [] } pub fn suggest_class (self , _arch : InlineAsmArch , _ty : InlineAsmType) -> Option < Self > { None } pub fn suggest_modifier (self , _arch : InlineAsmArch , _ty : InlineAsmType ,) -> Option < ModifierInfo > { None } pub fn default_modifier (self , _arch : InlineAsmArch) -> Option < ModifierInfo > { None } pub fn supported_types (self , arch : InlineAsmArch ,) -> & 'static [(InlineAsmType , Option < Symbol >)] { match self { Self :: reg => { if arch == InlineAsmArch :: Sparc { types ! { _ : I8 , I16 , I32 ; } } else { types ! { _ : I8 , I16 , I32 , I64 ; } } } Self :: yreg => & [] , } } }
};
}
