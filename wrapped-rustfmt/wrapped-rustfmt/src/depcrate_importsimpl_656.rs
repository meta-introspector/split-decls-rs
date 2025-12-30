// Generated macro for impl_656 (impl)
macro_rules! Depcrate_importsimpl_656 {
() => {
// Module: crate::imports
// Provides: {"impl_656"}
// Dependencies: {}
impl fmt :: Display for UseSegmentKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { UseSegmentKind :: Glob => write ! (f , "*") , UseSegmentKind :: Ident (ref s , Some (ref alias)) => write ! (f , "{s} as {alias}") , UseSegmentKind :: Ident (ref s , None) => write ! (f , "{s}") , UseSegmentKind :: Slf (..) => write ! (f , "self") , UseSegmentKind :: Super (..) => write ! (f , "super") , UseSegmentKind :: Crate (..) => write ! (f , "crate") , UseSegmentKind :: List (ref list) => { write ! (f , "{{") ? ; for (i , item) in list . iter () . enumerate () { if i != 0 { write ! (f , ", ") ? ; } write ! (f , "{item}") ? ; } write ! (f , "}}") } } } }
};
}
