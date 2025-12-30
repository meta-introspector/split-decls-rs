// Generated macro for high_byte (function)
macro_rules! Depcrate_asm_x86high_byte {
() => {
// Module: crate::asm::x86
// Provides: {"high_byte"}
// Dependencies: {}
fn high_byte (arch : InlineAsmArch , _reloc_model : RelocModel , _target_features : & FxIndexSet < Symbol > , _target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { match arch { InlineAsmArch :: X86_64 => Err ("high byte registers cannot be used as an operand on x86_64") , _ => Ok (()) , } }
};
}
