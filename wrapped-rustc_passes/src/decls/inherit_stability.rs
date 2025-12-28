macro_rules! inherit_stability {
    () => {
        fn inherit_stability (def_kind : DefKind) -> bool { match def_kind { DefKind :: Field | DefKind :: Variant | DefKind :: Ctor (..) => true , _ => false , } }
    };
}

inherit_stability!()