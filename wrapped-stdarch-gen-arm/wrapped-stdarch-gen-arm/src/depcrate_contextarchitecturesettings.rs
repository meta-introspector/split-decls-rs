// Generated macro for ArchitectureSettings (struct)
macro_rules! Depcrate_contextArchitectureSettings {
() => {
// Module: crate::context
// Provides: {"ArchitectureSettings"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct ArchitectureSettings { # [serde (alias = "arch")] pub arch_name : String , pub target_feature : Vec < String > , # [serde (alias = "llvm_prefix")] pub llvm_link_prefix : String , }
};
}
