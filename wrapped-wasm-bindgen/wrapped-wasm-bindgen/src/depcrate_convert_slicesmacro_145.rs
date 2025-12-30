// Generated macro for macro_145 (macro)
macro_rules! Depcrate_convert_slicesmacro_145 {
() => {
// Module: crate::convert::slices
// Provides: {"macro_145"}
// Dependencies: {}
cfg_if ! { if # [cfg (feature = "enable-interning")] { # [inline] fn unsafe_get_cached_str (x : & str) -> Option < WasmSlice > { crate :: cache :: intern :: unsafe_get_str (x) . map (| x | WasmSlice { ptr : 0 , len : x }) } } else { # [inline] fn unsafe_get_cached_str (_x : & str) -> Option < WasmSlice > { None } } }
};
}
