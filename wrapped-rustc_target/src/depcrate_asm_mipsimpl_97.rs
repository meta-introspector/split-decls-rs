// Generated macro for impl_97 (impl)
macro_rules! Depcrate_asm_mipsimpl_97 {
() => {
// Module: crate::asm::mips
// Provides: {"impl_97"}
// Dependencies: {}
impl MipsInlineAsmRegClass { pub fn valid_modifiers (self , _arch : super :: InlineAsmArch) -> & 'static [char] { & [] } pub fn suggest_class (self , _arch : InlineAsmArch , _ty : InlineAsmType) -> Option < Self > { None } pub fn suggest_modifier (self , _arch : InlineAsmArch , _ty : InlineAsmType ,) -> Option < ModifierInfo > { None } pub fn default_modifier (self , _arch : InlineAsmArch) -> Option < ModifierInfo > { None } pub fn supported_types (self , arch : InlineAsmArch ,) -> & 'static [(InlineAsmType , Option < Symbol >)] { match (self , arch) { (Self :: reg , InlineAsmArch :: Mips64) => types ! { _ : I8 , I16 , I32 , I64 , F32 , F64 ; } , (Self :: reg , _) => types ! { _ : I8 , I16 , I32 , F32 ; } , (Self :: freg , _) => types ! { _ : F32 , F64 ; } , } } }
};
}
