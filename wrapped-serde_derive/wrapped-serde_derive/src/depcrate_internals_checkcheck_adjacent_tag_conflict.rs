// Generated macro for check_adjacent_tag_conflict (function)
macro_rules! Depcrate_internals_checkcheck_adjacent_tag_conflict {
() => {
// Module: crate::internals::check
// Provides: {"check_adjacent_tag_conflict"}
// Dependencies: {}
fn check_adjacent_tag_conflict (cx : & Ctxt , cont : & Container) { let (type_tag , content_tag) = match cont . attrs . tag () { TagType :: Adjacent { tag , content } => (tag , content) , TagType :: Internal { .. } | TagType :: External | TagType :: None => return , } ; if type_tag == content_tag { cx . error_spanned_by (cont . original , format ! ("enum tags `{}` for type and content conflict with each other" , type_tag) ,) ; } }
};
}
