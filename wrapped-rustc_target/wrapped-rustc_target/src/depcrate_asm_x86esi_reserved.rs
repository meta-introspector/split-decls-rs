// Generated macro for esi_reserved (function)
macro_rules! Depcrate_asm_x86esi_reserved {
() => {
// Module: crate::asm::x86
// Provides: {"esi_reserved"}
// Dependencies: {}
fn esi_reserved (arch : InlineAsmArch , _reloc_model : RelocModel , _target_features : & FxIndexSet < Symbol > , _target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { match arch { InlineAsmArch :: X86 => { Err ("esi is used internally by LLVM and cannot be used as an operand for inline asm") } InlineAsmArch :: X86_64 => Ok (()) , _ => unreachable ! () , } }
};
}
