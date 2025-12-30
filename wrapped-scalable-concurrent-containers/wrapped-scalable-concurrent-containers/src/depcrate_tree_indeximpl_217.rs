// Generated macro for impl_217 (impl)
macro_rules! Depcrate_tree_indeximpl_217 {
() => {
// Module: crate::tree_index
// Provides: {"impl_217"}
// Dependencies: {}
impl < 'g , K , V > Iterator for Iter < '_ , 'g , K , V > where K : 'static + Clone + Ord , V : 'static + Clone , { type Item = (& 'g K , & 'g V) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if self . leaf_scanner . is_none () { let root_ptr = self . root . load (Acquire , self . guard) ; if let Some (root_ref) = root_ptr . as_ref () { if let Some (scanner) = root_ref . min (self . guard) { self . leaf_scanner . replace (scanner) ; } } else { return None ; } } if let Some (mut scanner) = self . leaf_scanner . take () { let min_allowed_key = scanner . get () . map (| (key , _) | key) ; if let Some (result) = scanner . next () { self . leaf_scanner . replace (scanner) ; return Some (result) ; } if let Some (new_scanner) = scanner . jump (min_allowed_key , self . guard) { if let Some (entry) = new_scanner . get () { self . leaf_scanner . replace (new_scanner) ; return Some (entry) ; } } } None } }
};
}
