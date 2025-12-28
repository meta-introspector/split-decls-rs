macro_rules! deps {
    () => {
        Stability!();
        ImpliedFeatures!();
    };
}

macro_rules! HEXAGON_FEATURES {
    () => {
        deps!();
        const HEXAGON_FEATURES : & [(& str , Stability , ImpliedFeatures)] = & [("hvx" , Unstable (sym :: hexagon_target_feature) , & []) , ("hvx-length128b" , Unstable (sym :: hexagon_target_feature) , & ["hvx"]) ,] ;
    };
}

HEXAGON_FEATURES!();