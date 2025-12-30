// Generated macro for impl_463 (impl)
macro_rules! Depcrate_jsonimpl_463 {
() => {
// Module: crate::json
// Provides: {"impl_463"}
// Dependencies: {}
impl < T : ToString , A : ToJson > ToJson for BTreeMap < T , A > { fn to_json (& self) -> Json { let mut d = Map :: new () ; for (key , value) in self { d . insert (key . to_string () , value . to_json ()) ; } Json :: Object (d) } }
};
}
