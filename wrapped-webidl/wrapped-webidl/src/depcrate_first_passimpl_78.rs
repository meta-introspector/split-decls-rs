// Generated macro for impl_78 (impl)
macro_rules! Depcrate_first_passimpl_78 {
() => {
// Module: crate::first_pass
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , () > for weedle :: CallbackDefinition < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , _ : ()) -> Result < () > { record . callbacks . insert (self . identifier . 0) ; Ok (()) } }
};
}
