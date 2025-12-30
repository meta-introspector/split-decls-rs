// Generated macro for AllocJobResult (enum)
macro_rules! Depcrate_distAllocJobResult {
() => {
// Module: crate::dist
// Provides: {"AllocJobResult"}
// Dependencies: {}
# [derive (Clone , Serialize , Deserialize)] # [serde (deny_unknown_fields)] pub enum AllocJobResult { Success { job_alloc : JobAlloc , need_toolchain : bool , } , Fail { msg : String , } , }
};
}
