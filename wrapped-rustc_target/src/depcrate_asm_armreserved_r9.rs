// Generated macro for reserved_r9 (function)
macro_rules! Depcrate_asm_armreserved_r9 {
() => {
// Module: crate::asm::arm
// Provides: {"reserved_r9"}
// Dependencies: {}
fn reserved_r9 (arch : InlineAsmArch , reloc_model : RelocModel , target_features : & FxIndexSet < Symbol > , target : & Target , is_clobber : bool ,) -> Result < () , & 'static str > { not_thumb1 (arch , reloc_model , target_features , target , is_clobber) ? ; match reloc_model { RelocModel :: Rwpi | RelocModel :: RopiRwpi => { Err ("the RWPI static base register (r9) cannot be used as an operand for inline asm") } _ => Ok (()) , } }
};
}
