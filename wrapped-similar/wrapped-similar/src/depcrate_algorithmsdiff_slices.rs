// Generated macro for diff_slices (function)
macro_rules! Depcrate_algorithmsdiff_slices {
() => {
// Module: crate::algorithms
// Provides: {"diff_slices"}
// Dependencies: {}
# [doc = " Shortcut for diffing slices with a specific algorithm."] pub fn diff_slices < D , T > (alg : Algorithm , d : & mut D , old : & [T] , new : & [T]) -> Result < () , D :: Error > where D : DiffHook , T : Eq + Hash + Ord , { diff (alg , d , old , 0 .. old . len () , new , 0 .. new . len ()) }
};
}
