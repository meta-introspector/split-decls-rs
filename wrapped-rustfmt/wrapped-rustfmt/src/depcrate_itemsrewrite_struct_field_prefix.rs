// Generated macro for rewrite_struct_field_prefix (function)
macro_rules! Depcrate_itemsrewrite_struct_field_prefix {
() => {
// Module: crate::items
// Provides: {"rewrite_struct_field_prefix"}
// Dependencies: {}
pub (crate) fn rewrite_struct_field_prefix (context : & RewriteContext < '_ > , field : & ast :: FieldDef ,) -> RewriteResult { let vis = format_visibility (context , & field . vis) ; let safety = format_safety (field . safety) ; let type_annotation_spacing = type_annotation_spacing (context . config) ; Ok (match field . ident { Some (name) => format ! ("{vis}{safety}{}{}:" , rewrite_ident (context , name) , type_annotation_spacing . 0) , None => format ! ("{vis}{safety}") , }) }
};
}
