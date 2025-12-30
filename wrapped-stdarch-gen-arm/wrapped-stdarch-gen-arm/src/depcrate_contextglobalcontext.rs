// Generated macro for GlobalContext (struct)
macro_rules! Depcrate_contextGlobalContext {
() => {
// Module: crate::context
// Provides: {"GlobalContext"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct GlobalContext { pub arch_cfgs : Vec < ArchitectureSettings > , # [serde (default)] pub uses_neon_types : bool , # [doc = " Should the yaml file automagically generate big endian shuffling"] # [serde (default)] pub auto_big_endian : Option < bool > , # [doc = " Should all LLVM wrappers convert their arguments to a signed type"] # [serde (default)] pub auto_llvm_sign_conversion : bool , }
};
}
