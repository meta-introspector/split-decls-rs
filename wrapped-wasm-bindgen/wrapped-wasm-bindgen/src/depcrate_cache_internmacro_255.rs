// Generated macro for macro_255 (macro)
macro_rules! Depcrate_cache_internmacro_255 {
() => {
// Module: crate::cache::intern
// Provides: {"macro_255"}
// Dependencies: {}
cfg_if ! { if # [cfg (feature = "enable-interning")] { use std :: thread_local ; use std :: string :: String ; use std :: borrow :: ToOwned ; use std :: cell :: RefCell ; use std :: collections :: HashMap ; use crate :: JsValue ; struct Cache { entries : RefCell < HashMap < String , JsValue >>, } thread_local ! { static CACHE : Cache = Cache { entries : RefCell :: new (HashMap :: new ()) , } ; } # [doc = " This returns the raw index of the cached JsValue, so you must take care"] # [doc = " so that you don't use it after it is freed."] pub (crate) fn unsafe_get_str (s : & str) -> Option < u32 > { CACHE . with (| cache | { let cache = cache . entries . borrow () ; cache . get (s) . map (| x | x . idx) }) } fn intern_str (key : & str) { CACHE . with (| cache | { let entries = & cache . entries ; if ! entries . borrow () . contains_key (key) { let value = JsValue :: from (key) ; entries . borrow_mut () . insert (key . to_owned () , value) ; } }) } fn unintern_str (key : & str) { CACHE . with (| cache | { let mut cache = cache . entries . borrow_mut () ; cache . remove (key) ; }) } } }
};
}
