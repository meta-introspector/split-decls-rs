// Generated macro for impl_54 (impl)
macro_rules! Depcrate_first_passimpl_54 {
() => {
// Module: crate::first_pass
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , () > for weedle :: IncludesStatementDefinition < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , () : ()) -> Result < () > { if util :: is_chrome_only (& self . attributes) { return Ok (()) ; } record . includes . entry (self . lhs_identifier . 0) . or_default () . insert (self . rhs_identifier . 0) ; Ok (()) } }
};
}
