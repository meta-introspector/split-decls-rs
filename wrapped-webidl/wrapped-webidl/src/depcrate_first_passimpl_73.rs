// Generated macro for impl_73 (impl)
macro_rules! Depcrate_first_passimpl_73 {
() => {
// Module: crate::first_pass
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , () > for weedle :: TypedefDefinition < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , () : ()) -> Result < () > { if util :: is_chrome_only (& self . attributes) { return Ok (()) ; } if record . typedefs . insert (self . identifier . 0 , & self . type_ . type_) . is_some () { log :: info ! ("Encountered multiple typedef declarations: {}" , self . identifier . 0) ; } Ok (()) } }
};
}
