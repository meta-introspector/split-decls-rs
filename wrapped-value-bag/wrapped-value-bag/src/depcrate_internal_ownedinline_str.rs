// Generated macro for inline_str (module)
macro_rules! Depcrate_internal_ownedinline_str {
() => {
// Module: crate::internal::owned
// Provides: {"inline_str"}
// Dependencies: {}
# [cfg (feature = "inline-str")] mod inline_str { use crate :: std :: { fmt , slice , str } ; pub (super) const MAX_INLINE_LEN : usize = 22 ; # [derive (Clone , Copy)] pub (crate) struct InlineStr { data : [u8 ; MAX_INLINE_LEN] , len : u8 , } impl fmt :: Debug for InlineStr { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (self . get () , f) } } impl fmt :: Display for InlineStr { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (self . get () , f) } } impl InlineStr { pub (super) fn copy_from (str : & str) -> Self { let str = str . as_bytes () ; let mut data = [0 ; MAX_INLINE_LEN] ; data [.. str . len ()] . copy_from_slice (str) ; InlineStr { data , len : str . len () as u8 , } } pub (super) const fn get (& self) -> & str { unsafe { str :: from_utf8_unchecked (slice :: from_raw_parts (& self . data as * const u8 , self . len as usize ,)) } } } # [cfg (test)] mod tests { use super :: * ; use std :: string :: ToString ; # [test] fn inline_str () { let s = InlineStr :: copy_from ("abc") ; assert_eq ! ("abc" , s . get ()) ; assert_eq ! ("abc" , s . to_string ()) ; } } }
};
}
