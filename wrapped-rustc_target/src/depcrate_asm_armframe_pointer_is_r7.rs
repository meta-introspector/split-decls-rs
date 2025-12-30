// Generated macro for frame_pointer_is_r7 (function)
macro_rules! Depcrate_asm_armframe_pointer_is_r7 {
() => {
// Module: crate::asm::arm
// Provides: {"frame_pointer_is_r7"}
// Dependencies: {}
fn frame_pointer_is_r7 (target_features : & FxIndexSet < Symbol > , target : & Target) -> bool { target . is_like_darwin || (! target . is_like_windows && target_features . contains (& sym :: thumb_mode)) }
};
}
