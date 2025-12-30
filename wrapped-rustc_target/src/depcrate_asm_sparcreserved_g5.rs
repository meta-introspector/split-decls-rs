// Generated macro for reserved_g5 (function)
macro_rules! Depcrate_asm_sparcreserved_g5 {
() => {
// Module: crate::asm::sparc
// Provides: {"reserved_g5"}
// Dependencies: {}
fn reserved_g5 (arch : InlineAsmArch , _reloc_model : RelocModel , _target_features : & FxIndexSet < Symbol > , _target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { if arch == InlineAsmArch :: Sparc { Err ("g5 is reserved for system on SPARC32") } else { Ok (()) } }
};
}
