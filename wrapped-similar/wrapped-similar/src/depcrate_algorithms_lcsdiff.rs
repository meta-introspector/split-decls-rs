// Generated macro for diff (function)
macro_rules! Depcrate_algorithms_lcsdiff {
() => {
// Module: crate::algorithms::lcs
// Provides: {"diff"}
// Dependencies: {}
# [doc = " LCS diff algorithm."] # [doc = ""] # [doc = " Diff `old`, between indices `old_range` and `new` between indices `new_range`."] # [doc = ""] # [doc = " This diff is done with an optional deadline that defines the maximal"] # [doc = " execution time permitted before it bails and falls back to an very bad"] # [doc = " approximation.  Deadlines with LCS do not make a lot of sense and should"] # [doc = " not be used."] pub fn diff < Old , New , D > (d : & mut D , old : & Old , old_range : Range < usize > , new : & New , new_range : Range < usize > ,) -> Result < () , D :: Error > where Old : Index < usize > + ? Sized , New : Index < usize > + ? Sized , D : DiffHook , New :: Output : PartialEq < Old :: Output > , { diff_deadline (d , old , old_range , new , new_range , None) }
};
}
