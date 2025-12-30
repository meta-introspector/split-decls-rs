// Generated macro for impl_79 (impl)
macro_rules! Depcrate_first_passimpl_79 {
() => {
// Module: crate::first_pass
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , () > for weedle :: CallbackInterfaceDefinition < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , _ : ()) -> Result < () > { if util :: is_chrome_only (& self . attributes) { return Ok (()) ; } if self . inheritance . is_some () { log :: warn ! ("skipping callback interface with inheritance: {}" , self . identifier . 0) ; return Ok (()) ; } let data = CallbackInterfaceData { definition : self , single_function : self . members . body . len () == 1 , } ; record . callback_interfaces . insert (self . identifier . 0 , data) ; Ok (()) } }
};
}
