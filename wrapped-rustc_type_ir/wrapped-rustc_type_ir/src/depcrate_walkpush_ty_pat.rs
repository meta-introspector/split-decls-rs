// Generated macro for push_ty_pat (function)
macro_rules! Depcrate_walkpush_ty_pat {
() => {
// Module: crate::walk
// Provides: {"push_ty_pat"}
// Dependencies: {}
fn push_ty_pat < I : Interner > (stack : & mut TypeWalkerStack < I > , pat : I :: Pat) { match pat . kind () { ty :: PatternKind :: Range { start , end } => { stack . push (end . into ()) ; stack . push (start . into ()) ; } ty :: PatternKind :: Or (pats) => { for pat in pats . iter () { push_ty_pat :: < I > (stack , pat) } } } }
};
}
