// Generated macro for get_block_child_shape (function)
macro_rules! Depcrate_chainsget_block_child_shape {
() => {
// Module: crate::chains
// Provides: {"get_block_child_shape"}
// Dependencies: {}
fn get_block_child_shape (prev_ends_with_block : bool , context : & RewriteContext < '_ > , shape : Shape ,) -> Shape { if prev_ends_with_block { shape . block_indent (0) } else { shape . block_indent (context . config . tab_spaces ()) } . with_max_width (context . config) }
};
}
