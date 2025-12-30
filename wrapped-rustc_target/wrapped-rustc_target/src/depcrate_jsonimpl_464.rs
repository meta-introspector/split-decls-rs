// Generated macro for impl_464 (impl)
macro_rules! Depcrate_jsonimpl_464 {
() => {
// Module: crate::json
// Provides: {"impl_464"}
// Dependencies: {}
impl < A : ToJson > ToJson for Option < A > { fn to_json (& self) -> Json { match * self { None => Json :: Null , Some (ref value) => value . to_json () , } } }
};
}
