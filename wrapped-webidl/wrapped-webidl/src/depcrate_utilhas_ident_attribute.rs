// Generated macro for has_ident_attribute (function)
macro_rules! Depcrate_utilhas_ident_attribute {
() => {
// Module: crate::util
// Provides: {"has_ident_attribute"}
// Dependencies: {}
fn has_ident_attribute (list : Option < & ExtendedAttributeList > , ident : & str) -> bool { let list = match list { Some (list) => list , None => return false , } ; list . body . list . iter () . any (| attr | match attr { ExtendedAttribute :: Ident (id) => id . lhs_identifier . 0 == ident , ExtendedAttribute :: IdentList (id) => id . identifier . 0 == ident , _ => false , }) }
};
}
