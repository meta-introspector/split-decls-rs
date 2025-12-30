// Generated macro for optimal_buf_size (function)
macro_rules! Depcrate_filters_fsoptimal_buf_size {
() => {
// Module: crate::filters::fs
// Provides: {"optimal_buf_size"}
// Dependencies: {}
fn optimal_buf_size (metadata : & Metadata) -> usize { let block_size = get_block_size (metadata) ; cmp :: min (block_size as u64 , metadata . len ()) as usize }
};
}
