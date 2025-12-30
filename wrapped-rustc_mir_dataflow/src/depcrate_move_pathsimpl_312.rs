// Generated macro for impl_312 (impl)
macro_rules! Depcrate_move_pathsimpl_312 {
() => {
// Module: crate::move_paths
// Provides: {"impl_312"}
// Dependencies: {}
impl < T > Index < Location > for LocationMap < T > { type Output = T ; fn index (& self , index : Location) -> & Self :: Output { & self . map [index . block] [index . statement_index] } }
};
}
