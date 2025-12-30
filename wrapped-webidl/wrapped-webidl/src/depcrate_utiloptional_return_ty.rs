// Generated macro for optional_return_ty (function)
macro_rules! Depcrate_utiloptional_return_ty {
() => {
// Module: crate::util
// Provides: {"optional_return_ty"}
// Dependencies: {}
# [doc = " Wrap [`TypePosition::Return`] type into an `Option` if not already and if not a `JsValue`."] pub fn optional_return_ty (ty : syn :: Type) -> syn :: Type { if let syn :: Type :: Path (path) = & ty { if let Some (segment) = path . path . segments . first () { if segment . ident == "Option" { return ty ; } else if path . path . leading_colon . is_some () && segment . ident == "wasm_bindgen" { if let Some (segment) = path . path . segments . iter () . nth (1) { if segment . ident == "JsValue" { return ty ; } } } } } option_ty (ty) }
};
}
