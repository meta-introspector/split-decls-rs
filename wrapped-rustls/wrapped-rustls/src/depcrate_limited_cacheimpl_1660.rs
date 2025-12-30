// Generated macro for impl_1660 (impl)
macro_rules! Depcrate_limited_cacheimpl_1660 {
() => {
// Module: crate::limited_cache
// Provides: {"impl_1660"}
// Dependencies: {}
impl < K , V > LimitedCache < K , V > where K : Eq + Hash + Clone + core :: fmt :: Debug , V : Default , { pub (crate) fn get_or_insert_default_and_edit (& mut self , k : K , edit : impl FnOnce (& mut V)) { let inserted_new_item = match self . map . entry (k) { Entry :: Occupied (value) => { edit (value . into_mut ()) ; false } entry @ Entry :: Vacant (_) => { self . oldest . push_back (entry . key () . clone ()) ; edit (entry . or_insert_with (V :: default)) ; true } } ; if inserted_new_item && self . oldest . capacity () == self . oldest . len () { if let Some (oldest_key) = self . oldest . pop_front () { self . map . remove (& oldest_key) ; } } } pub (crate) fn get_mut < Q : Hash + Eq + ? Sized > (& mut self , k : & Q) -> Option < & mut V > where K : Borrow < Q > , { self . map . get_mut (k) } }
};
}
