// Generated macro for diff (function)
macro_rules! Depcrate_algorithms_myersdiff {
() => {
// Module: crate::algorithms::myers
// Provides: {"diff"}
// Dependencies: {}
# [doc = " Myers' diff algorithm."] # [doc = ""] # [doc = " Diff `old`, between indices `old_range` and `new` between indices `new_range`."] pub fn diff < Old , New , D > (d : & mut D , old : & Old , old_range : Range < usize > , new : & New , new_range : Range < usize > ,) -> Result < () , D :: Error > where Old : Index < usize > + ? Sized , New : Index < usize > + ? Sized , D : DiffHook , New :: Output : PartialEq < Old :: Output > , { diff_deadline (d , old , old_range , new , new_range , None) }
};
}
