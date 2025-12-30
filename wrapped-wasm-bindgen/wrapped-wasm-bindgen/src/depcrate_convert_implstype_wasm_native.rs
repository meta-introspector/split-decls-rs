// Generated macro for type_wasm_native (macro)
macro_rules! Depcrate_convert_implstype_wasm_native {
() => {
// Module: crate::convert::impls
// Provides: {"type_wasm_native"}
// Dependencies: {}
macro_rules ! type_wasm_native { ($ ($ t : tt as $ c : tt) *) => ($ (impl IntoWasmAbi for $ t { type Abi = $ c ; # [inline] fn into_abi (self) -> $ c { self as $ c } } impl FromWasmAbi for $ t { type Abi = $ c ; # [inline] unsafe fn from_abi (js : $ c) -> Self { js as $ t } } impl IntoWasmAbi for Option <$ t > { type Abi = Option <$ c >; # [inline] fn into_abi (self) -> Self :: Abi { self . map (| v | v as $ c) } } impl FromWasmAbi for Option <$ t > { type Abi = Option <$ c >; # [inline] unsafe fn from_abi (js : Self :: Abi) -> Self { js . map (| v : $ c | v as $ t) } }) *) }
};
}
