// Generated macro for associated_item_from_impl_item (function)
macro_rules! Depcrate_assocassociated_item_from_impl_item {
() => {
// Module: crate::assoc
// Provides: {"associated_item_from_impl_item"}
// Dependencies: {}
fn associated_item_from_impl_item (tcx : TyCtxt < '_ > , impl_item : & hir :: ImplItem < '_ >) -> ty :: AssocItem { let owner_id = impl_item . owner_id ; let name = impl_item . ident . name ; let kind = match impl_item . kind { hir :: ImplItemKind :: Const { .. } => ty :: AssocKind :: Const { name } , hir :: ImplItemKind :: Fn { .. } => { ty :: AssocKind :: Fn { name , has_self : fn_has_self_parameter (tcx , owner_id) } } hir :: ImplItemKind :: Type { .. } => { ty :: AssocKind :: Type { data : ty :: AssocTypeData :: Normal (name) } } } ; let container = match impl_item . impl_kind { ImplItemImplKind :: Inherent { .. } => ty :: AssocContainer :: InherentImpl , ImplItemImplKind :: Trait { trait_item_def_id , .. } => { ty :: AssocContainer :: TraitImpl (trait_item_def_id) } } ; ty :: AssocItem { kind , def_id : owner_id . to_def_id () , container } }
};
}
