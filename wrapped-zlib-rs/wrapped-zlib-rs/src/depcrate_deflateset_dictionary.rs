// Generated macro for set_dictionary (function)
macro_rules! Depcrate_deflateset_dictionary {
() => {
// Module: crate::deflate
// Provides: {"set_dictionary"}
// Dependencies: {}
pub fn set_dictionary (stream : & mut DeflateStream , mut dictionary : & [u8]) -> ReturnCode { let state = & mut stream . state ; let wrap = state . wrap ; if wrap == 2 || (wrap == 1 && state . status != Status :: Init) || state . lookahead != 0 { return ReturnCode :: StreamError ; } if wrap == 1 { stream . adler = adler32 (stream . adler as u32 , dictionary) as z_checksum ; } state . wrap = 0 ; if dictionary . len () >= state . window . capacity () { if wrap == 0 { state . head . as_mut_slice () . fill (0) ; state . strstart = 0 ; state . block_start = 0 ; state . insert = 0 ; } else { } dictionary = & dictionary [dictionary . len () - state . w_size ..] ; } let avail = stream . avail_in ; let next = stream . next_in ; stream . avail_in = dictionary . len () as _ ; stream . next_in = dictionary . as_ptr () as * mut u8 ; fill_window (stream) ; while stream . state . lookahead >= STD_MIN_MATCH { let str = stream . state . strstart ; let n = stream . state . lookahead - (STD_MIN_MATCH - 1) ; stream . state . insert_string (str , n) ; stream . state . strstart = str + n ; stream . state . lookahead = STD_MIN_MATCH - 1 ; fill_window (stream) ; } let state = & mut stream . state ; state . strstart += state . lookahead ; state . block_start = state . strstart as _ ; state . insert = state . lookahead ; state . lookahead = 0 ; state . prev_length = 0 ; state . match_available = false ; stream . next_in = next ; stream . avail_in = avail ; state . wrap = wrap ; ReturnCode :: Ok }
};
}
