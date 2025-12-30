// Generated macro for impl_105 (impl)
macro_rules! Depcrate_asm_msp430impl_105 {
() => {
// Module: crate::asm::msp430
// Provides: {"impl_105"}
// Dependencies: {}
impl Msp430InlineAsmRegClass { pub fn valid_modifiers (self , _arch : super :: InlineAsmArch) -> & 'static [char] { & [] } pub fn suggest_class (self , _arch : InlineAsmArch , _ty : InlineAsmType) -> Option < Self > { None } pub fn suggest_modifier (self , _arch : InlineAsmArch , _ty : InlineAsmType ,) -> Option < ModifierInfo > { None } pub fn default_modifier (self , _arch : InlineAsmArch) -> Option < ModifierInfo > { None } pub fn supported_types (self , arch : InlineAsmArch ,) -> & 'static [(InlineAsmType , Option < Symbol >)] { match (self , arch) { (Self :: reg , _) => types ! { _ : I8 , I16 ; } , } } }
};
}
