// Generated macro for impl_30 (impl)
macro_rules! Depcrate_linesimpl_30 {
() => {
// Module: crate::lines
// Provides: {"impl_30"}
// Dependencies: {}
impl LineStep { # [doc = " Create a new line iterator over the given range of bytes using the"] # [doc = " given line terminator."] # [doc = ""] # [doc = " Callers should provide the actual bytes for each call to `next`. The"] # [doc = " same slice must be provided to each call."] # [doc = ""] # [doc = " This panics if `start` is not less than or equal to `end`."] pub fn new (line_term : u8 , start : usize , end : usize) -> LineStep { LineStep { line_term , pos : start , end } } # [doc = " Return the start and end position of the next line in the given bytes."] # [doc = ""] # [doc = " The caller must past exactly the same slice of bytes for each call to"] # [doc = " `next`."] # [doc = ""] # [doc = " The range returned includes the line terminator. Ranges are always"] # [doc = " non-empty."] pub fn next (& mut self , bytes : & [u8]) -> Option < (usize , usize) > { self . next_impl (bytes) } # [doc = " Like next, but returns a `Match` instead of a tuple."] # [inline (always)] pub (crate) fn next_match (& mut self , bytes : & [u8]) -> Option < Match > { self . next_impl (bytes) . map (| (s , e) | Match :: new (s , e)) } # [inline (always)] fn next_impl (& mut self , mut bytes : & [u8]) -> Option < (usize , usize) > { bytes = & bytes [.. self . end] ; match bytes [self . pos ..] . find_byte (self . line_term) { None => { if self . pos < bytes . len () { let m = (self . pos , bytes . len ()) ; assert ! (m . 0 <= m . 1) ; self . pos = m . 1 ; Some (m) } else { None } } Some (line_end) => { let m = (self . pos , self . pos + line_end + 1) ; assert ! (m . 0 <= m . 1) ; self . pos = m . 1 ; Some (m) } } } }
};
}
