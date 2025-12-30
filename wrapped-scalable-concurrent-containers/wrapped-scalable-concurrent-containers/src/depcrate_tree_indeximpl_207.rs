// Generated macro for impl_207 (impl)
macro_rules! Depcrate_tree_indeximpl_207 {
() => {
// Module: crate::tree_index
// Provides: {"impl_207"}
// Dependencies: {}
impl < K , V > TreeIndex < K , V > { # [doc = " Creates an empty [`TreeIndex`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::TreeIndex;"] # [doc = ""] # [doc = " let treeindex: TreeIndex<u64, u32> = TreeIndex::new();"] # [doc = " ```"] # [cfg (not (feature = "loom"))] # [inline] # [must_use] pub const fn new () -> Self { Self { root : AtomicShared :: null () , } } # [doc = " Creates an empty [`TreeIndex`]."] # [cfg (feature = "loom")] # [inline] # [must_use] pub fn new () -> Self { Self { root : AtomicShared :: null () , } } # [doc = " Clears the [`TreeIndex`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::TreeIndex;"] # [doc = ""] # [doc = " let treeindex: TreeIndex<u64, u32> = TreeIndex::new();"] # [doc = ""] # [doc = " treeindex.clear();"] # [doc = " assert_eq!(treeindex.len(), 0);"] # [doc = " ```"] # [inline] pub fn clear (& self) { if let (Some (root) , _) = self . root . swap ((None , Tag :: None) , Acquire) { root . clear (& Guard :: new ()) ; } } # [doc = " Returns the depth of the [`TreeIndex`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use scc::TreeIndex;"] # [doc = ""] # [doc = " let treeindex: TreeIndex<u64, u32> = TreeIndex::new();"] # [doc = " assert_eq!(treeindex.depth(), 0);"] # [doc = " ```"] # [inline] # [must_use] pub fn depth (& self) -> usize { let guard = Guard :: new () ; self . root . load (Acquire , & guard) . as_ref () . map_or (0 , | root_ref | root_ref . depth (1 , & guard)) } }
};
}
