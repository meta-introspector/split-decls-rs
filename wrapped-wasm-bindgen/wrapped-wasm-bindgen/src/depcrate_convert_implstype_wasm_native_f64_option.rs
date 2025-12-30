// Generated macro for type_wasm_native_f64_option (macro)
macro_rules! Depcrate_convert_implstype_wasm_native_f64_option {
() => {
// Module: crate::convert::impls
// Provides: {"type_wasm_native_f64_option"}
// Dependencies: {}
macro_rules ! type_wasm_native_f64_option { ($ ($ t : tt as $ c : tt) *) => ($ (impl IntoWasmAbi for $ t { type Abi = $ c ; # [inline] fn into_abi (self) -> $ c { self as $ c } } impl FromWasmAbi for $ t { type Abi = $ c ; # [inline] unsafe fn from_abi (js : $ c) -> Self { js as $ t } } impl IntoWasmAbi for Option <$ t > { type Abi = f64 ; # [inline] fn into_abi (self) -> Self :: Abi { self . map (| v | v as $ c as f64) . unwrap_or (F64_ABI_OPTION_SENTINEL) } } impl FromWasmAbi for Option <$ t > { type Abi = f64 ; # [inline] unsafe fn from_abi (js : Self :: Abi) -> Self { if js == F64_ABI_OPTION_SENTINEL { None } else { Some (js as $ c as $ t) } } }) *) }
};
}
