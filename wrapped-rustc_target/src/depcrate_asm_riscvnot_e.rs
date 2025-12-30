// Generated macro for not_e (function)
macro_rules! Depcrate_asm_riscvnot_e {
() => {
// Module: crate::asm::riscv
// Provides: {"not_e"}
// Dependencies: {}
fn not_e (_arch : InlineAsmArch , _reloc_model : RelocModel , target_features : & FxIndexSet < Symbol > , _target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { if is_e (target_features) { Err ("register can't be used with the `e` target feature") } else { Ok (()) } }
};
}
