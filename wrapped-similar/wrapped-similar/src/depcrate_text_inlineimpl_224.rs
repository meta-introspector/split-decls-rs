// Generated macro for impl_224 (impl)
macro_rules! Depcrate_text_inlineimpl_224 {
() => {
// Module: crate::text::inline
// Provides: {"impl_224"}
// Dependencies: {}
impl < 'bufs , 's , T : DiffableStr + ? Sized > MultiLookup < 'bufs , 's , T > { fn new (strings : & 'bufs [& 's T]) -> MultiLookup < 'bufs , 's , T > { let mut seqs = Vec :: new () ; for (string_idx , string) in strings . iter () . enumerate () { let mut offset = 0 ; let iter = { # [cfg (feature = "unicode")] { string . tokenize_unicode_words () } # [cfg (not (feature = "unicode"))] { string . tokenize_words () } } ; for word in iter { seqs . push ((word , string_idx , offset)) ; offset += word . len () ; } } MultiLookup { strings , seqs } } pub fn len (& self) -> usize { self . seqs . len () } fn get_original_slices (& self , idx : usize , len : usize) -> Vec < (usize , & 's T) > { let mut last = None ; let mut rv = Vec :: new () ; for offset in 0 .. len { let (s , str_idx , char_idx) = self . seqs [idx + offset] ; last = match last { None => Some ((str_idx , char_idx , s . len ())) , Some ((last_str_idx , start_char_idx , last_len)) => { if last_str_idx == str_idx { Some ((str_idx , start_char_idx , last_len + s . len ())) } else { rv . push ((last_str_idx , self . strings [last_str_idx] . slice (start_char_idx .. start_char_idx + last_len) ,)) ; Some ((str_idx , char_idx , s . len ())) } } } ; } if let Some ((str_idx , start_char_idx , len)) = last { rv . push ((str_idx , self . strings [str_idx] . slice (start_char_idx .. start_char_idx + len) ,)) ; } rv } }
};
}
