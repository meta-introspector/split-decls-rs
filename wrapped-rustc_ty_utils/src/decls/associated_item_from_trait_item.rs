macro_rules! associated_item_from_trait_item {
    () => {
        fn associated_item_from_trait_item (tcx : TyCtxt < '_ > , trait_item : & hir :: TraitItem < '_ > ,) -> ty :: AssocItem { let owner_id = trait_item . owner_id ; let name = trait_item . ident . name ; let kind = match trait_item . kind { hir :: TraitItemKind :: Const { .. } => ty :: AssocKind :: Const { name } , hir :: TraitItemKind :: Fn { .. } => { ty :: AssocKind :: Fn { name , has_self : fn_has_self_parameter (tcx , owner_id) } } hir :: TraitItemKind :: Type { .. } => { ty :: AssocKind :: Type { data : ty :: AssocTypeData :: Normal (name) } } } ; ty :: AssocItem { kind , def_id : owner_id . to_def_id () , container : ty :: AssocContainer :: Trait } }
    };
}

associated_item_from_trait_item!();