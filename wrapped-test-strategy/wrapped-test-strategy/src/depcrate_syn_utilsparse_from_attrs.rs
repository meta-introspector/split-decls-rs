// Generated macro for parse_from_attrs (function)
macro_rules! Depcrate_syn_utilsparse_from_attrs {
() => {
// Module: crate::syn_utils
// Provides: {"parse_from_attrs"}
// Dependencies: {}
pub fn parse_from_attrs < T : Parse + Default > (attrs : & [Attribute] , name : & str) -> Result < T > { let mut a = None ; for attr in attrs { if attr . path () . is_ident (name) { if a . is_some () { bail ! (attr . span () , "attribute `{}` can specified only once" , name) ; } a = Some (attr) ; } } if let Some (a) = a { a . parse_args () } else { Ok (T :: default ()) } }
};
}
