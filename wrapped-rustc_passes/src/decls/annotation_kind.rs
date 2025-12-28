macro_rules! deps {
    () => {
        AnnotationKind!();
    };
}

macro_rules! annotation_kind {
    () => {
        deps!();
        fn annotation_kind (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> AnnotationKind { let def_kind = tcx . def_kind (def_id) ; match def_kind { DefKind :: Impl { of_trait : false } | DefKind :: ForeignMod => AnnotationKind :: Container , DefKind :: Impl { of_trait : true } => AnnotationKind :: DeprecationProhibited , DefKind :: TyParam | DefKind :: ConstParam => { match & tcx . hir_node_by_def_id (def_id) . expect_generic_param () . kind { hir :: GenericParamKind :: Type { default : Some (_) , .. } | hir :: GenericParamKind :: Const { default : Some (_) , .. } => { AnnotationKind :: Container } _ => AnnotationKind :: Prohibited , } } DefKind :: AssocTy | DefKind :: AssocFn | DefKind :: AssocConst => { match tcx . def_kind (tcx . local_parent (def_id)) { DefKind :: Impl { of_trait : true } => AnnotationKind :: Prohibited , _ => AnnotationKind :: Required , } } _ => AnnotationKind :: Required , } }
    };
}

annotation_kind!();