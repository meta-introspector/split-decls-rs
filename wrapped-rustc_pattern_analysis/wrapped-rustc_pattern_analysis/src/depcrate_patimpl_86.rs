// Generated macro for impl_86 (impl)
macro_rules! Depcrate_patimpl_86 {
() => {
// Module: crate::pat
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'p , Cx : PatCx > Clone for PatOrWild < 'p , Cx > { fn clone (& self) -> Self { match self { PatOrWild :: Wild => PatOrWild :: Wild , PatOrWild :: Pat (pat) => PatOrWild :: Pat (pat) , } } }
};
}
