// Generated macro for x86_64_only (function)
macro_rules! Depcrate_asm_x86x86_64_only {
() => {
// Module: crate::asm::x86
// Provides: {"x86_64_only"}
// Dependencies: {}
fn x86_64_only (arch : InlineAsmArch , _reloc_model : RelocModel , _target_features : & FxIndexSet < Symbol > , _target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { match arch { InlineAsmArch :: X86 => Err ("register is only available on x86_64") , InlineAsmArch :: X86_64 => Ok (()) , _ => unreachable ! () , } }
};
}
