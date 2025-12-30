// Generated macro for UhfBackend (trait)
macro_rules! DepcrateUhfBackend {
() => {
// Module: crate
// Provides: {"UhfBackend"}
// Dependencies: {}
# [doc = " Trait implemented by UHF backends."] pub trait UhfBackend : ParBlocksSizeUser { # [doc = " Process single block."] fn proc_block (& mut self , block : & Block < Self >) ; # [doc = " Process several blocks in parallel."] # [inline (always)] fn proc_par_blocks (& mut self , blocks : & ParBlocks < Self >) { for block in blocks { self . proc_block (block) ; } } # [doc = " Returns the number of blocks that should be passed to `Self::proc_block` before"] # [doc = " `Self::proc_par_blocks` can be used efficiently. This is always less than"] # [doc = " `Self::ParBlocksSize`."] fn blocks_needed_to_align (& self) -> usize { 0 } }
};
}
