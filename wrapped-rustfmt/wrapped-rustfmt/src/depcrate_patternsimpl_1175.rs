// Generated macro for impl_1175 (impl)
macro_rules! Depcrate_patternsimpl_1175 {
() => {
// Module: crate::patterns
// Provides: {"impl_1175"}
// Dependencies: {}
impl < 'a > TuplePatField < 'a > { fn is_dotdot (& self) -> bool { match self { TuplePatField :: Pat (pat) => matches ! (pat . kind , ast :: PatKind :: Rest) , TuplePatField :: Dotdot (_) => true , } } }
};
}
