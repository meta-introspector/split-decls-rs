// Generated macro for impl_722 (impl)
macro_rules! Depcrate_specimpl_722 {
() => {
// Module: crate::spec
// Provides: {"impl_722"}
// Dependencies: {}
impl ToJson for LinkSelfContainedDefault { fn to_json (& self) -> Json { match * self { LinkSelfContainedDefault :: WithComponents (components) => { let mut map = BTreeMap :: new () ; map . insert ("components" , components) ; map . to_json () } LinkSelfContainedDefault :: True => "true" . to_json () , LinkSelfContainedDefault :: False => "false" . to_json () , LinkSelfContainedDefault :: InferredForMusl => "musl" . to_json () , LinkSelfContainedDefault :: InferredForMingw => "mingw" . to_json () , } } }
};
}
