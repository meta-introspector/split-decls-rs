// Generated macro for lm_init (function)
macro_rules! Depcrate_deflatelm_init {
() => {
// Module: crate::deflate
// Provides: {"lm_init"}
// Dependencies: {}
fn lm_init (state : & mut State) { state . window_size = 2 * state . w_size ; state . head . as_mut_slice () . fill (0) ; lm_set_level (state , state . level) ; state . strstart = 0 ; state . block_start = 0 ; state . lookahead = 0 ; state . insert = 0 ; state . prev_length = 0 ; state . match_available = false ; state . match_start = 0 ; state . ins_h = 0 ; }
};
}
