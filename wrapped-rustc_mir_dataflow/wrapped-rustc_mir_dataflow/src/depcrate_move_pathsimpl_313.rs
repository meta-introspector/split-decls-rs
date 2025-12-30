// Generated macro for impl_313 (impl)
macro_rules! Depcrate_move_pathsimpl_313 {
() => {
// Module: crate::move_paths
// Provides: {"impl_313"}
// Dependencies: {}
impl < T > IndexMut < Location > for LocationMap < T > { fn index_mut (& mut self , index : Location) -> & mut Self :: Output { & mut self . map [index . block] [index . statement_index] } }
};
}
