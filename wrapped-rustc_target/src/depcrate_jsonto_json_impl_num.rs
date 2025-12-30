// Generated macro for to_json_impl_num (macro)
macro_rules! Depcrate_jsonto_json_impl_num {
() => {
// Module: crate::json
// Provides: {"to_json_impl_num"}
// Dependencies: {}
macro_rules ! to_json_impl_num { ($ ($ t : ty) , +) => ($ (impl ToJson for $ t { fn to_json (& self) -> Json { Json :: Number (Number :: from (* self)) } }) +) }
};
}
