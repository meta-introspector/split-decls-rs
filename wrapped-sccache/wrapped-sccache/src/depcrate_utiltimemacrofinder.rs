// Generated macro for TimeMacroFinder (struct)
macro_rules! Depcrate_utilTimeMacroFinder {
() => {
// Module: crate::util
// Provides: {"TimeMacroFinder"}
// Dependencies: {}
# [doc = " Used during the chunked hashing process to check for C preprocessor time"] # [doc = " macros (namely `__TIMESTAMP__`, `__DATE__`, `__DATETIME__`) while reusing"] # [doc = " the same buffer as the hashing function, for efficiency."] # [doc = ""] # [doc = " See `[Self::find_time_macros]` for details."] # [derive (Debug , Default)] pub struct TimeMacroFinder { found_date : Cell < bool > , found_time : Cell < bool > , found_timestamp : Cell < bool > , overlap_buffer : [u8 ; MAX_HAYSTACK_LEN * 2] , # [doc = " Counter of chunks of full size we've been through. Partial reads do"] # [doc = " not count and are handled separately."] full_chunks_counter : usize , # [doc = " Contents of the previous read if it was smaller than `MAX_HAYSTACK_LEN`,"] # [doc = " plus MAX_HAYSTACK_LEN bytes of the previous chunk, to account for"] # [doc = " the possibility of partial reads splitting a time macro"] # [doc = " across two calls."] previous_small_read : Vec < u8 > , }
};
}
