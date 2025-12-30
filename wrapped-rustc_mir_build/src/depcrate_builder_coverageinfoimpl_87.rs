// Generated macro for impl_87 (impl)
macro_rules! Depcrate_builder_coverageinfoimpl_87 {
() => {
// Module: crate::builder::coverageinfo
// Provides: {"impl_87"}
// Dependencies: {}
impl BlockMarkerGen { fn next_block_marker_id (& mut self) -> BlockMarkerId { let id = BlockMarkerId :: from_usize (self . num_block_markers) ; self . num_block_markers += 1 ; id } fn inject_block_marker (& mut self , cfg : & mut CFG < '_ > , source_info : SourceInfo , block : BasicBlock ,) -> BlockMarkerId { let id = self . next_block_marker_id () ; let marker_statement = mir :: Statement :: new (source_info , mir :: StatementKind :: Coverage (CoverageKind :: BlockMarker { id }) ,) ; cfg . push (block , marker_statement) ; id } }
};
}
