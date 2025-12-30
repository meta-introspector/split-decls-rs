// Generated macro for impl_223 (impl)
macro_rules! Depcrate_tree_indeximpl_223 {
() => {
// Module: crate::tree_index
// Provides: {"impl_223"}
// Dependencies: {}
impl < 'g , K , V , Q , R > Iterator for Range < '_ , 'g , K , V , Q , R > where K : 'static + Clone + Ord , V : 'static + Clone , Q : Comparable < K > + ? Sized , R : RangeBounds < Q > , { type Item = (& 'g K , & 'g V) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { while let Some ((k , v)) = self . next_unbounded () { if self . check_lower_bound { match self . bounds . start_bound () { Excluded (key) => { if key . compare (k) . is_ge () { continue ; } } Included (key) => { if key . compare (k) . is_gt () { continue ; } } Unbounded => () , } } self . check_lower_bound = false ; if self . check_upper_bound { match self . bounds . end_bound () { Excluded (key) => { if key . compare (k) . is_gt () { return Some ((k , v)) ; } } Included (key) => { if key . compare (k) . is_ge () { return Some ((k , v)) ; } } Unbounded => { return Some ((k , v)) ; } } break ; } return Some ((k , v)) ; } None } }
};
}
