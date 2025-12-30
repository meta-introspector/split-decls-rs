// Generated macro for target_reserves_x18 (function)
macro_rules! Depcrate_asm_aarch64target_reserves_x18 {
() => {
// Module: crate::asm::aarch64
// Provides: {"target_reserves_x18"}
// Dependencies: {}
pub (crate) fn target_reserves_x18 (target : & Target , target_features : & FxIndexSet < Symbol >) -> bool { target . os == "android" || target . os == "fuchsia" || target . env == "ohos" || target . is_like_darwin || target . is_like_windows || target_features . contains (& sym :: reserve_x18) }
};
}
