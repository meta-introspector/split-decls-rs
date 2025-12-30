// Generated macro for flag (macro)
macro_rules! Depcrate_compiler_argsflag {
() => {
// Module: crate::compiler::args
// Provides: {"flag"}
// Dependencies: {}
# [doc = " Helper macro used to define ArgInfo::Flag's."] # [doc = " Variant is an enum variant, e.g. enum ArgType { Variant }"] # [doc = "     flag!(\"-foo\", Variant)"] macro_rules ! flag { ($ s : expr , $ variant : expr) => { ArgInfo :: Flag ($ s , $ variant) } ; }
};
}
