macro_rules! target_from_impl_item {
    () => {
        fn target_from_impl_item < 'tcx > (tcx : TyCtxt < 'tcx > , impl_item : & hir :: ImplItem < '_ >) -> Target { match impl_item . kind { hir :: ImplItemKind :: Const (..) => Target :: AssocConst , hir :: ImplItemKind :: Fn (..) => { let parent_def_id = tcx . hir_get_parent_item (impl_item . hir_id ()) . def_id ; let containing_item = tcx . hir_expect_item (parent_def_id) ; let containing_impl_is_for_trait = match & containing_item . kind { hir :: ItemKind :: Impl (impl_) => impl_ . of_trait . is_some () , _ => bug ! ("parent of an ImplItem must be an Impl") , } ; if containing_impl_is_for_trait { Target :: Method (MethodKind :: Trait { body : true }) } else { Target :: Method (MethodKind :: Inherent) } } hir :: ImplItemKind :: Type (..) => Target :: AssocTy , } }
    };
}

target_from_impl_item!();