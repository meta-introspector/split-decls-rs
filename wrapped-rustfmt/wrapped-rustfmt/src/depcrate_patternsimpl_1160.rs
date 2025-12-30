// Generated macro for impl_1160 (impl)
macro_rules! Depcrate_patternsimpl_1160 {
() => {
// Module: crate::patterns
// Provides: {"impl_1160"}
// Dependencies: {}
impl < 'a > TuplePatField < 'a > { fn is_dotdot (& self) -> bool { match self { TuplePatField :: Pat (pat) => matches ! (pat . kind , ast :: PatKind :: Rest) , TuplePatField :: Dotdot (_) => true , } } }
};
}
