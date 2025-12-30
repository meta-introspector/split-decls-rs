// Generated macro for macro_36 (macro)
macro_rules! Depcrate_listablemacro_36 {
() => {
// Module: crate::listable
// Provides: {"macro_36"}
// Dependencies: {}
collection ! { # [cfg (feature = "alloc")] (T : Valuable) alloc :: collections :: LinkedList < T >, # [cfg (feature = "alloc")] (T : Valuable + Ord) alloc :: collections :: BinaryHeap < T >, # [cfg (feature = "alloc")] (T : Valuable + Ord) alloc :: collections :: BTreeSet < T >, # [cfg (feature = "std")] (T : Valuable + Eq + std :: hash :: Hash , H : std :: hash :: BuildHasher) std :: collections :: HashSet < T , H >, }
};
}
