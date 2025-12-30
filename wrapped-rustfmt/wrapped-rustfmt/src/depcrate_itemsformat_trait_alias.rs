// Generated macro for format_trait_alias (function)
macro_rules! Depcrate_itemsformat_trait_alias {
() => {
// Module: crate::items
// Provides: {"format_trait_alias"}
// Dependencies: {}
pub (crate) fn format_trait_alias (context : & RewriteContext < '_ > , ident : Ident , item : & ast :: Item , generics : & ast :: Generics , generic_bounds : & ast :: GenericBounds , shape : Shape ,) -> RewriteResult { let ast :: Item { ref vis , span , .. } = * item ; let alias = rewrite_ident (context , ident) ; let g_shape = shape . offset_left (6 , span) ? . sub_width (2 , span) ? ; let generics_str = rewrite_generics (context , alias , generics , g_shape) ? ; let vis_str = format_visibility (context , vis) ; let lhs = format ! ("{vis_str}trait {generics_str} =") ; let trait_alias_bounds = TraitAliasBounds { generic_bounds , generics , } ; let result = rewrite_assign_rhs (context , lhs , & trait_alias_bounds , & RhsAssignKind :: Bounds , shape . sub_width (1 , generics . span) ? ,) ? ; Ok (result + ";") }
};
}
