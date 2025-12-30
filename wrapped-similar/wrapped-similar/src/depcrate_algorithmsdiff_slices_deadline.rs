// Generated macro for diff_slices_deadline (function)
macro_rules! Depcrate_algorithmsdiff_slices_deadline {
() => {
// Module: crate::algorithms
// Provides: {"diff_slices_deadline"}
// Dependencies: {}
# [doc = " Shortcut for diffing slices with a specific algorithm."] pub fn diff_slices_deadline < D , T > (alg : Algorithm , d : & mut D , old : & [T] , new : & [T] , deadline : Option < Instant > ,) -> Result < () , D :: Error > where D : DiffHook , T : Eq + Hash + Ord , { diff_deadline (alg , d , old , 0 .. old . len () , new , 0 .. new . len () , deadline) }
};
}
