// Generated macro for impl_1661 (impl)
macro_rules! Depcrate_limited_cacheimpl_1661 {
() => {
// Module: crate::limited_cache
// Provides: {"impl_1661"}
// Dependencies: {}
impl < K , V > LimitedCache < K , V > where K : Eq + Hash + Clone + core :: fmt :: Debug , V : Default , { # [doc = " Create a new LimitedCache with the given rough capacity."] pub (crate) fn new (capacity_order_of_magnitude : usize) -> Self { Self { map : HashMap :: with_capacity (capacity_order_of_magnitude) , oldest : VecDeque :: with_capacity (capacity_order_of_magnitude) , } } pub (crate) fn insert (& mut self , k : K , v : V) { let inserted_new_item = match self . map . entry (k) { Entry :: Occupied (mut old) => { old . insert (v) ; false } entry @ Entry :: Vacant (_) => { self . oldest . push_back (entry . key () . clone ()) ; entry . or_insert (v) ; true } } ; if inserted_new_item && self . oldest . capacity () == self . oldest . len () { if let Some (oldest_key) = self . oldest . pop_front () { self . map . remove (& oldest_key) ; } } } pub (crate) fn get < Q : Hash + Eq + ? Sized > (& self , k : & Q) -> Option < & V > where K : Borrow < Q > , { self . map . get (k) } pub (crate) fn remove < Q : Hash + Eq + ? Sized > (& mut self , k : & Q) -> Option < V > where K : Borrow < Q > , { let value = self . map . remove (k) ? ; if let Some (index) = self . oldest . iter () . position (| item | item . borrow () == k) { self . oldest . remove (index) ; } Some (value) } }
};
}
