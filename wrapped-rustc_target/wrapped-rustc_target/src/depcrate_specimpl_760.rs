// Generated macro for impl_760 (impl)
macro_rules! Depcrate_specimpl_760 {
() => {
// Module: crate::spec
// Provides: {"impl_760"}
// Dependencies: {}
impl ToJson for StackProbeType { fn to_json (& self) -> Json { Json :: Object (match self { StackProbeType :: None => { [(String :: from ("kind") , "none" . to_json ())] . into_iter () . collect () } StackProbeType :: Inline => { [(String :: from ("kind") , "inline" . to_json ())] . into_iter () . collect () } StackProbeType :: Call => { [(String :: from ("kind") , "call" . to_json ())] . into_iter () . collect () } StackProbeType :: InlineOrCall { min_llvm_version_for_inline : (maj , min , patch) } => [(String :: from ("kind") , "inline-or-call" . to_json ()) , (String :: from ("min-llvm-version-for-inline") , Json :: Array (vec ! [maj . to_json () , min . to_json () , patch . to_json ()]) ,) ,] . into_iter () . collect () , }) } }
};
}
