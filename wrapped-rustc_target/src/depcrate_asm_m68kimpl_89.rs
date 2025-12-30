// Generated macro for impl_89 (impl)
macro_rules! Depcrate_asm_m68kimpl_89 {
() => {
// Module: crate::asm::m68k
// Provides: {"impl_89"}
// Dependencies: {}
impl M68kInlineAsmRegClass { pub fn valid_modifiers (self , _arch : super :: InlineAsmArch) -> & 'static [char] { & [] } pub fn suggest_class (self , _arch : InlineAsmArch , _ty : InlineAsmType) -> Option < Self > { None } pub fn suggest_modifier (self , _arch : InlineAsmArch , _ty : InlineAsmType ,) -> Option < ModifierInfo > { None } pub fn default_modifier (self , _arch : InlineAsmArch) -> Option < ModifierInfo > { None } pub fn supported_types (self , _arch : InlineAsmArch ,) -> & 'static [(InlineAsmType , Option < Symbol >)] { match self { Self :: reg => types ! { _ : I16 , I32 ; } , Self :: reg_data => types ! { _ : I8 , I16 , I32 ; } , Self :: reg_addr => types ! { _ : I16 , I32 ; } , } } }
};
}
