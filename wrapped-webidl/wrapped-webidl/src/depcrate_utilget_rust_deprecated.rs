// Generated macro for get_rust_deprecated (function)
macro_rules! Depcrate_utilget_rust_deprecated {
() => {
// Module: crate::util
// Provides: {"get_rust_deprecated"}
// Dependencies: {}
pub fn get_rust_deprecated (ext_attrs : & Option < ExtendedAttributeList >) -> Option < Option < String > > { ext_attrs . as_ref () ? . body . list . iter () . filter_map (| attr | match attr { ExtendedAttribute :: NoArgs (ExtendedAttributeNoArgs (id)) => Some ((id , None)) , ExtendedAttribute :: Ident (ExtendedAttributeIdent { lhs_identifier : id , rhs , .. }) => Some ((id , Some (rhs))) , _ => None , }) . filter (| (id , _) | id . 0 == "RustDeprecated") . find_map (| (_ , rhs) | match rhs { None => Some (None) , Some (IdentifierOrString :: String (s)) => Some (Some (s . 0 . to_owned ())) , _ => unimplemented ! () , }) }
};
}
