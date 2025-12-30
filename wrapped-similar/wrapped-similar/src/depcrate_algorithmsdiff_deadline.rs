// Generated macro for diff_deadline (function)
macro_rules! Depcrate_algorithmsdiff_deadline {
() => {
// Module: crate::algorithms
// Provides: {"diff_deadline"}
// Dependencies: {}
# [doc = " Creates a diff between old and new with the given algorithm with deadline."] # [doc = ""] # [doc = " Diffs `old`, between indices `old_range` and `new` between indices `new_range`."] # [doc = ""] # [doc = " This diff is done with an optional deadline that defines the maximal"] # [doc = " execution time permitted before it bails and falls back to an approximation."] # [doc = " Note that not all algorithms behave well if they reach the deadline (LCS"] # [doc = " for instance produces a very simplistic diff when the deadline is reached"] # [doc = " in all cases)."] pub fn diff_deadline < Old , New , D > (alg : Algorithm , d : & mut D , old : & Old , old_range : Range < usize > , new : & New , new_range : Range < usize > , deadline : Option < Instant > ,) -> Result < () , D :: Error > where Old : Index < usize > + ? Sized , New : Index < usize > + ? Sized , D : DiffHook , Old :: Output : Hash + Eq + Ord , New :: Output : PartialEq < Old :: Output > + Hash + Eq + Ord , { match alg { Algorithm :: Myers => myers :: diff_deadline (d , old , old_range , new , new_range , deadline) , Algorithm :: Patience => patience :: diff_deadline (d , old , old_range , new , new_range , deadline) , Algorithm :: Lcs => lcs :: diff_deadline (d , old , old_range , new , new_range , deadline) , } }
};
}
