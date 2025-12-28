macro_rules! deps {
    () => {
        UnnecessaryStableFeature!();
    };
}

macro_rules! unnecessary_stable_feature_lint {
    () => {
        deps!();
        fn unnecessary_stable_feature_lint (tcx : TyCtxt < '_ > , span : Span , feature : Symbol , mut since : Symbol ,) { if since . as_str () == VERSION_PLACEHOLDER { since = sym :: env_CFG_RELEASE ; } tcx . emit_node_span_lint (lint :: builtin :: STABLE_FEATURES , hir :: CRATE_HIR_ID , span , errors :: UnnecessaryStableFeature { feature , since } ,) ; }
    };
}

unnecessary_stable_feature_lint!()