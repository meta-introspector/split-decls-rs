// Generated macro for impl_744 (impl)
macro_rules! Depcrate_specimpl_744 {
() => {
// Module: crate::spec
// Provides: {"impl_744"}
// Dependencies: {}
impl ToJson for SmallDataThresholdSupport { fn to_json (& self) -> Value { match self { Self :: None => "none" . to_json () , Self :: DefaultForArch => "default-for-arch" . to_json () , Self :: LlvmModuleFlag (flag) => format ! ("llvm-module-flag={flag}") . to_json () , Self :: LlvmArg (arg) => format ! ("llvm-arg={arg}") . to_json () , } } }
};
}
