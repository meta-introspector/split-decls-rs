// Generated macro for impl_81 (impl)
macro_rules! Depcrate_asm_loongarchimpl_81 {
() => {
// Module: crate::asm::loongarch
// Provides: {"impl_81"}
// Dependencies: {}
impl LoongArchInlineAsmRegClass { pub fn valid_modifiers (self , _arch : super :: InlineAsmArch) -> & 'static [char] { & [] } pub fn suggest_class (self , _arch : InlineAsmArch , _ty : InlineAsmType) -> Option < Self > { None } pub fn suggest_modifier (self , _arch : InlineAsmArch , _ty : InlineAsmType ,) -> Option < ModifierInfo > { None } pub fn default_modifier (self , _arch : InlineAsmArch) -> Option < ModifierInfo > { None } pub fn supported_types (self , arch : InlineAsmArch ,) -> & 'static [(InlineAsmType , Option < Symbol >)] { match (self , arch) { (Self :: reg , InlineAsmArch :: LoongArch64) => { types ! { _ : I8 , I16 , I32 , I64 , F16 , F32 , F64 ; } } (Self :: reg , InlineAsmArch :: LoongArch32) => types ! { _ : I8 , I16 , I32 , F16 , F32 ; } , (Self :: freg , _) => types ! { f : F16 , F32 ; d : F64 ; } , _ => unreachable ! ("unsupported register class") , } } }
};
}
