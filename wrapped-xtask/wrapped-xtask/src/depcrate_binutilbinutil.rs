// Generated macro for binutil (function)
macro_rules! Depcrate_binutilbinutil {
() => {
// Module: crate::binutil
// Provides: {"binutil"}
// Dependencies: {}
pub fn binutil (name : & str) -> Option < PathBuf > { static LLVM_TOOLS : LazyLock < LlvmTools > = LazyLock :: new (| | LlvmTools :: new () . unwrap ()) ; LLVM_TOOLS . tool (name) }
};
}
