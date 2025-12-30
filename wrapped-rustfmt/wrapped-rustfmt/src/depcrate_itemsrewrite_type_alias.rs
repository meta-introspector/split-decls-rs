// Generated macro for rewrite_type_alias (function)
macro_rules! Depcrate_itemsrewrite_type_alias {
() => {
// Module: crate::items
// Provides: {"rewrite_type_alias"}
// Dependencies: {}
pub (crate) fn rewrite_type_alias < 'a > (ty_alias_kind : & ast :: TyAlias , vis : & ast :: Visibility , context : & RewriteContext < 'a > , indent : Indent , visitor_kind : ItemVisitorKind , span : Span ,) -> RewriteResult { use ItemVisitorKind :: * ; let ast :: TyAlias { defaultness , ident , ref generics , ref bounds , ref ty , where_clauses , } = * ty_alias_kind ; let ty_opt = ty . as_ref () ; let rhs_hi = ty . as_ref () . map_or (where_clauses . before . span . hi () , | ty | ty . span . hi ()) ; let rw_info = & TyAliasRewriteInfo (context , indent , generics , where_clauses , ident , span) ; let op_ty = opaque_ty (ty) ; match (visitor_kind , & op_ty) { (Item | AssocTraitItem | ForeignItem , Some (op_bounds)) => { let op = OpaqueType { bounds : op_bounds } ; rewrite_ty (rw_info , Some (bounds) , Some (& op) , rhs_hi , vis) } (Item | AssocTraitItem | ForeignItem , None) => { rewrite_ty (rw_info , Some (bounds) , ty_opt , rhs_hi , vis) } (AssocImplItem , _) => { let result = if let Some (op_bounds) = op_ty { let op = OpaqueType { bounds : op_bounds } ; rewrite_ty (rw_info , Some (bounds) , Some (& op) , rhs_hi , & DEFAULT_VISIBILITY ,) } else { rewrite_ty (rw_info , Some (bounds) , ty_opt , rhs_hi , vis) } ? ; match defaultness { ast :: Defaultness :: Default (..) => Ok (format ! ("default {result}")) , _ => Ok (result) , } } } }
};
}
