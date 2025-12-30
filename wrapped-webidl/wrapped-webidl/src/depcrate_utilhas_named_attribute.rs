// Generated macro for has_named_attribute (function)
macro_rules! Depcrate_utilhas_named_attribute {
() => {
// Module: crate::util
// Provides: {"has_named_attribute"}
// Dependencies: {}
# [doc = " Search for an attribute by name in some webidl object's attributes."] fn has_named_attribute (list : Option < & ExtendedAttributeList > , attribute : & str) -> bool { let list = match list { Some (list) => list , None => return false , } ; list . body . list . iter () . any (| attr | match attr { ExtendedAttribute :: NoArgs (name) => (name . 0) . 0 == attribute , _ => false , }) }
};
}
