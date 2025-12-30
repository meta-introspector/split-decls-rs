// Generated macro for impl_150 (impl)
macro_rules! Depcrate_udiffimpl_150 {
() => {
// Module: crate::udiff
// Provides: {"impl_150"}
// Dependencies: {}
impl < 'diff , 'old , 'new , 'bufs , T : DiffableStr + ? Sized > fmt :: Display for UnifiedDiff < 'diff , 'old , 'new , 'bufs , T > where 'diff : 'old + 'new + 'bufs , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut header = self . header . as_ref () ; for hunk in self . iter_hunks () { if let Some ((old_file , new_file)) = header . take () { writeln ! (f , "--- {}" , old_file) ? ; writeln ! (f , "+++ {}" , new_file) ? ; } write ! (f , "{}" , hunk) ? ; } Ok (()) } }
};
}
