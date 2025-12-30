// Generated macro for split_block (function)
macro_rules! Depcrate_check_pointerssplit_block {
() => {
// Module: crate::check_pointers
// Provides: {"split_block"}
// Dependencies: {}
fn split_block (basic_blocks : & mut IndexVec < BasicBlock , BasicBlockData < '_ > > , location : Location ,) -> BasicBlock { let block_data = & mut basic_blocks [location . block] ; let new_block = BasicBlockData :: new_stmts (block_data . statements . split_off (location . statement_index) , block_data . terminator . take () , block_data . is_cleanup ,) ; basic_blocks . push (new_block) }
};
}
