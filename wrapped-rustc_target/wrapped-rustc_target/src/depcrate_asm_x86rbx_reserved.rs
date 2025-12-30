// Generated macro for rbx_reserved (function)
macro_rules! Depcrate_asm_x86rbx_reserved {
() => {
// Module: crate::asm::x86
// Provides: {"rbx_reserved"}
// Dependencies: {}
fn rbx_reserved (arch : InlineAsmArch , _reloc_model : RelocModel , _target_features : & FxIndexSet < Symbol > , _target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { match arch { InlineAsmArch :: X86 => Ok (()) , InlineAsmArch :: X86_64 => { Err ("rbx is used internally by LLVM and cannot be used as an operand for inline asm") } _ => unreachable ! () , } }
};
}
