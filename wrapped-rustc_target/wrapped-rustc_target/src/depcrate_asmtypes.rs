// Generated macro for types (macro)
macro_rules! Depcrate_asmtypes {
() => {
// Module: crate::asm
// Provides: {"types"}
// Dependencies: {}
macro_rules ! types { ($ (_ : $ ($ ty : expr) ,+;) ? $ ($ feature : ident : $ ($ ty2 : expr) ,+;) *) => { { use super :: InlineAsmType ::*; & [$ ($ (($ ty , None) ,) *) ? $ ($ (($ ty2 , Some (rustc_span :: sym ::$ feature)) ,) *) *] } } ; }
};
}
