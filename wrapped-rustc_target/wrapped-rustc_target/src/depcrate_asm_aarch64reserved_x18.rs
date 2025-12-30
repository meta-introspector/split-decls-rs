// Generated macro for reserved_x18 (function)
macro_rules! Depcrate_asm_aarch64reserved_x18 {
() => {
// Module: crate::asm::aarch64
// Provides: {"reserved_x18"}
// Dependencies: {}
fn reserved_x18 (_arch : InlineAsmArch , _reloc_model : RelocModel , target_features : & FxIndexSet < Symbol > , target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { if target_reserves_x18 (target , target_features) { Err ("x18 is a reserved register on this target") } else { Ok (()) } }
};
}
