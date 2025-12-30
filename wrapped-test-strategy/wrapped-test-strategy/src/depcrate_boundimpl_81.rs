// Generated macro for impl_81 (impl)
macro_rules! Depcrate_boundimpl_81 {
() => {
// Module: crate::bound
// Provides: {"impl_81"}
// Dependencies: {}
impl Parse for Bound { fn parse (input : ParseStream) -> Result < Self > { if input . peek (Token ! [..]) { return Ok (Self :: Default { _dotdot : input . parse () ? , }) ; } let fork = input . fork () ; match fork . parse () { Ok (p) => { input . advance_to (& fork) ; Ok (Self :: Predicate (p)) } Err (e) => { if let Ok (ty) = input . parse () { Ok (Self :: Type (ty)) } else { Err (e) } } } } }
};
}
