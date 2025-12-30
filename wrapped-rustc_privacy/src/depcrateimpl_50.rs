// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
impl VisibilityLike for ty :: Visibility { const MAX : Self = ty :: Visibility :: Public ; fn new_min < const SHALLOW : bool > (find : & FindMin < '_ , '_ , Self , SHALLOW > , def_id : LocalDefId ,) -> Self { min (find . tcx . local_visibility (def_id) , find . min , find . tcx) } }
};
}
