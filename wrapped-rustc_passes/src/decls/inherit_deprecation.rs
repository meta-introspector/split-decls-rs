macro_rules! inherit_deprecation {
    () => {
        fn inherit_deprecation (def_kind : DefKind) -> bool { match def_kind { DefKind :: LifetimeParam | DefKind :: TyParam | DefKind :: ConstParam => false , _ => true , } }
    };
}

inherit_deprecation!();