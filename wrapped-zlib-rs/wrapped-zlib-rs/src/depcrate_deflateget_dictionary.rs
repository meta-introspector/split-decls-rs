// Generated macro for get_dictionary (function)
macro_rules! Depcrate_deflateget_dictionary {
() => {
// Module: crate::deflate
// Provides: {"get_dictionary"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " The `dictionary` must have enough space for the dictionary."] pub unsafe fn get_dictionary (stream : & DeflateStream < '_ > , dictionary : * mut u8) -> usize { let s = & stream . state ; let len = Ord :: min (s . strstart + s . lookahead , s . w_size) ; if ! dictionary . is_null () && len > 0 { unsafe { core :: ptr :: copy_nonoverlapping (s . window . as_ptr () . add (s . strstart + s . lookahead - len) , dictionary , len ,) ; } } len }
};
}
