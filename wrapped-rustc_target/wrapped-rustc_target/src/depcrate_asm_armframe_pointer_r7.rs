// Generated macro for frame_pointer_r7 (function)
macro_rules! Depcrate_asm_armframe_pointer_r7 {
() => {
// Module: crate::asm::arm
// Provides: {"frame_pointer_r7"}
// Dependencies: {}
fn frame_pointer_r7 (_arch : InlineAsmArch , _reloc_model : RelocModel , target_features : & FxIndexSet < Symbol > , target : & Target , _is_clobber : bool ,) -> Result < () , & 'static str > { if frame_pointer_is_r7 (target_features , target) { Err ("the frame pointer (r7) cannot be used as an operand for inline asm") } else { Ok (()) } }
};
}
