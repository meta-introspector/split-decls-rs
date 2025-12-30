// Generated macro for CONTEXT (struct)
macro_rules! Depcrate_b_arch_dependenciesCONTEXT {
() => {
// Module: crate::b_arch_dependencies
// Provides: {"CONTEXT"}
// Dependencies: {}
# [repr (C)] # [cfg (target_arch = "aarch64")] # [derive (Clone , Copy)] pub struct CONTEXT { pub ContextFlags : CONTEXT_FLAGS , pub Cpsr : u32 , pub Anonymous : CONTEXT_0 , pub Sp : u64 , pub Pc : u64 , pub V : [ARM64_NT_NEON128 ; 32] , pub Fpcr : u32 , pub Fpsr : u32 , pub Bcr : [u32 ; 8] , pub Bvr : [u64 ; 8] , pub Wcr : [u32 ; 2] , pub Wvr : [u64 ; 2] , }
};
}
