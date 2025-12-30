// Generated macro for impl_53 (impl)
macro_rules! Depcrate_first_passimpl_53 {
() => {
// Module: crate::first_pass
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , ApiStability > for weedle :: EnumDefinition < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , stability : ApiStability ,) -> Result < () > { if util :: is_chrome_only (& self . attributes) { return Ok (()) ; } let enum_data = EnumData { definition : self , stability , } ; if record . enums . insert (self . identifier . 0 , enum_data) . is_some () { log :: info ! ("Encountered multiple enum declarations: {}" , self . identifier . 0) ; } Ok (()) } }
};
}
