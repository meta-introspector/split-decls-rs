// Generated macro for get_dictionary (function)
macro_rules! Depcrate_inflateget_dictionary {
() => {
// Module: crate::inflate
// Provides: {"get_dictionary"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " The `dictionary` must have enough space for the dictionary."] pub unsafe fn get_dictionary (stream : & InflateStream < '_ > , dictionary : * mut u8) -> usize { let whave = stream . state . window . have () ; let wnext = stream . state . window . next () ; if ! dictionary . is_null () { unsafe { core :: ptr :: copy_nonoverlapping (stream . state . window . as_ptr () . add (wnext) , dictionary , whave - wnext ,) ; core :: ptr :: copy_nonoverlapping (stream . state . window . as_ptr () , dictionary . add (whave) . sub (wnext) . cast () , wnext ,) ; } } stream . state . window . have () }
};
}
