// Generated macro for params (function)
macro_rules! Depcrate_deflateparams {
() => {
// Module: crate::deflate
// Provides: {"params"}
// Dependencies: {}
pub fn params (stream : & mut DeflateStream , level : i32 , strategy : Strategy) -> ReturnCode { let level = if level == crate :: c_api :: Z_DEFAULT_COMPRESSION { 6 } else { level } ; if ! (0 ..= 9) . contains (& level) { return ReturnCode :: StreamError ; } let level = level as i8 ; let func = CONFIGURATION_TABLE [stream . state . level as usize] . func ; let state = & mut stream . state ; # [allow (unpredictable_function_pointer_comparisons)] if (strategy != state . strategy || func != CONFIGURATION_TABLE [level as usize] . func) && state . last_flush != - 2 { let err = deflate (stream , DeflateFlush :: Block) ; if err == ReturnCode :: StreamError { return err ; } let state = & mut stream . state ; if stream . avail_in != 0 || ((state . strstart as isize - state . block_start) + state . lookahead as isize) != 0 { return ReturnCode :: BufError ; } } let state = & mut stream . state ; if state . level != level { if state . level == 0 && state . matches != 0 { if state . matches == 1 { self :: slide_hash :: slide_hash (state) ; } else { state . head . as_mut_slice () . fill (0) ; } state . matches = 0 ; } lm_set_level (state , level) ; } state . strategy = strategy ; ReturnCode :: Ok }
};
}
