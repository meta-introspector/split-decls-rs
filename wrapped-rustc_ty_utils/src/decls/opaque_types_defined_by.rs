macro_rules! deps {
    () => {
        OpaqueTypeCollector!();
    };
}

macro_rules! opaque_types_defined_by {
    () => {
        deps!();
        fn opaque_types_defined_by < 'tcx > (tcx : TyCtxt < 'tcx > , item : LocalDefId ,) -> & 'tcx ty :: List < LocalDefId > { let kind = tcx . def_kind (item) ; trace ! (? kind) ; let mut collector = OpaqueTypeCollector :: new (tcx , item) ; collector . collect_taits_from_defines_attr () ; super :: sig_types :: walk_types (tcx , item , & mut collector) ; match kind { DefKind :: AssocFn | DefKind :: Fn | DefKind :: Static { .. } | DefKind :: Const | DefKind :: AssocConst | DefKind :: AnonConst => { collector . collect_taits_declared_in_body () ; } DefKind :: Closure | DefKind :: InlineConst | DefKind :: SyntheticCoroutineBody => { collector . opaques . extend (tcx . opaque_types_defined_by (tcx . local_parent (item))) ; } DefKind :: AssocTy | DefKind :: TyAlias | DefKind :: GlobalAsm => { } DefKind :: OpaqueTy | DefKind :: Mod | DefKind :: Struct | DefKind :: Union | DefKind :: Enum | DefKind :: Variant | DefKind :: Trait | DefKind :: ForeignTy | DefKind :: TraitAlias | DefKind :: TyParam | DefKind :: ConstParam | DefKind :: Ctor (_ , _) | DefKind :: Macro (_) | DefKind :: ExternCrate | DefKind :: Use | DefKind :: ForeignMod | DefKind :: Field | DefKind :: LifetimeParam | DefKind :: Impl { .. } => { span_bug ! (tcx . def_span (item) , "`opaque_types_defined_by` not defined for {} `{item:?}`" , kind . descr (item . to_def_id ())) ; } } tcx . mk_local_def_ids (& collector . opaques) }
    };
}

opaque_types_defined_by!()