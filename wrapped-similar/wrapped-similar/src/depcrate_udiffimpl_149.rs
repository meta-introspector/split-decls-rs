// Generated macro for impl_149 (impl)
macro_rules! Depcrate_udiffimpl_149 {
() => {
// Module: crate::udiff
// Provides: {"impl_149"}
// Dependencies: {}
impl < 'diff , 'old , 'new , 'bufs , T : DiffableStr + ? Sized > fmt :: Display for UnifiedDiffHunk < 'diff , 'old , 'new , 'bufs , T > where 'diff : 'old + 'new + 'bufs , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { for (idx , change) in self . iter_changes () . enumerate () { if idx == 0 { writeln ! (f , "{}" , self . header ()) ? ; } write ! (f , "{}{}" , change . tag () , change . to_string_lossy ()) ? ; if ! self . diff . newline_terminated () { writeln ! (f) ? ; } if self . diff . newline_terminated () && change . missing_newline () { writeln ! (f , "{}" , MissingNewlineHint (self . missing_newline_hint)) ? ; } } Ok (()) } }
};
}
