// Generated macro for diff_deadline (function)
macro_rules! Depcrate_algorithms_myersdiff_deadline {
() => {
// Module: crate::algorithms::myers
// Provides: {"diff_deadline"}
// Dependencies: {}
# [doc = " Myers' diff algorithm with deadline."] # [doc = ""] # [doc = " Diff `old`, between indices `old_range` and `new` between indices `new_range`."] # [doc = ""] # [doc = " This diff is done with an optional deadline that defines the maximal"] # [doc = " execution time permitted before it bails and falls back to an approximation."] pub fn diff_deadline < Old , New , D > (d : & mut D , old : & Old , old_range : Range < usize > , new : & New , new_range : Range < usize > , deadline : Option < Instant > ,) -> Result < () , D :: Error > where Old : Index < usize > + ? Sized , New : Index < usize > + ? Sized , D : DiffHook , New :: Output : PartialEq < Old :: Output > , { let max_d = max_d (old_range . len () , new_range . len ()) ; let mut vb = V :: new (max_d) ; let mut vf = V :: new (max_d) ; conquer (d , old , old_range , new , new_range , & mut vf , & mut vb , deadline ,) ? ; d . finish () }
};
}
