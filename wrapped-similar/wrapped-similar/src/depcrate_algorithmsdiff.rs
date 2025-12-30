// Generated macro for diff (function)
macro_rules! Depcrate_algorithmsdiff {
() => {
// Module: crate::algorithms
// Provides: {"diff"}
// Dependencies: {}
# [doc = " Creates a diff between old and new with the given algorithm."] # [doc = ""] # [doc = " Diffs `old`, between indices `old_range` and `new` between indices `new_range`."] pub fn diff < Old , New , D > (alg : Algorithm , d : & mut D , old : & Old , old_range : Range < usize > , new : & New , new_range : Range < usize > ,) -> Result < () , D :: Error > where Old : Index < usize > + ? Sized , New : Index < usize > + ? Sized , D : DiffHook , Old :: Output : Hash + Eq + Ord , New :: Output : PartialEq < Old :: Output > + Hash + Eq + Ord , { diff_deadline (alg , d , old , old_range , new , new_range , None) }
};
}
