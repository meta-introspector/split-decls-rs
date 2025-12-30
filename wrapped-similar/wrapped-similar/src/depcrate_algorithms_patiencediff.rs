// Generated macro for diff (function)
macro_rules! Depcrate_algorithms_patiencediff {
() => {
// Module: crate::algorithms::patience
// Provides: {"diff"}
// Dependencies: {}
# [doc = " Patience diff algorithm."] # [doc = ""] # [doc = " Diff `old`, between indices `old_range` and `new` between indices `new_range`."] pub fn diff < Old , New , D > (d : & mut D , old : & Old , old_range : Range < usize > , new : & New , new_range : Range < usize > ,) -> Result < () , D :: Error > where Old : Index < usize > + ? Sized , New : Index < usize > + ? Sized , Old :: Output : Hash + Eq , New :: Output : PartialEq < Old :: Output > + Hash + Eq , D : DiffHook , { diff_deadline (d , old , old_range , new , new_range , None) }
};
}
