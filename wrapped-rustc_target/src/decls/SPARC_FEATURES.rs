macro_rules! deps {
    () => {
        ImpliedFeatures!();
        Stability!();
    };
}

macro_rules! SPARC_FEATURES {
    () => {
        deps!();
        const SPARC_FEATURES : & [(& str , Stability , ImpliedFeatures)] = & [("leoncasa" , Unstable (sym :: sparc_target_feature) , & []) , ("v8plus" , Unstable (sym :: sparc_target_feature) , & []) , ("v9" , Unstable (sym :: sparc_target_feature) , & []) ,] ;
    };
}

SPARC_FEATURES!();