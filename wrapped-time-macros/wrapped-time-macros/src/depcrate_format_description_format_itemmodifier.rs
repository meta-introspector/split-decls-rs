// Generated macro for modifier (macro)
macro_rules! Depcrate_format_description_format_itemmodifier {
() => {
// Module: crate::format_description::format_item
// Provides: {"modifier"}
// Dependencies: {}
macro_rules ! modifier { ($ (enum $ name : ident $ (($ target_ty : ty)) ? { $ ($ (# [$ attr : meta]) ? $ variant : ident $ (($ target_value : expr)) ? = $ parse_variant : literal) ,* $ (,) ? }) +) => { $ (# [derive (Default)] enum $ name { $ ($ (# [$ attr]) ? $ variant) ,* } impl $ name { # [doc = " Parse the modifier from its string representation."] fn from_modifier_value (value : & Spanned <& [u8] >) -> Result < Option < Self >, Error > { $ (if value . eq_ignore_ascii_case ($ parse_variant) { return Ok (Some (Self ::$ variant)) ; }) * Err (value . span . error ("invalid modifier value")) } } impl From <$ name > for target_ty ! ($ name $ ($ target_ty) ?) { fn from (modifier : $ name) -> Self { match modifier { $ ($ name ::$ variant => target_value ! ($ name $ variant $ ($ target_value) ?)) ,* } } }) + } ; }
};
}
