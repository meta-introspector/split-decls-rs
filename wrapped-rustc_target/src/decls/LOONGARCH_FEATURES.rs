macro_rules! deps {
    () => {
        Stability!();
        ImpliedFeatures!();
    };
}

macro_rules! LOONGARCH_FEATURES {
    () => {
        deps!();
        static LOONGARCH_FEATURES : & [(& str , Stability , ImpliedFeatures)] = & [("32s" , Unstable (sym :: loongarch_target_feature) , & []) , ("d" , Stable , & ["f"]) , ("div32" , Unstable (sym :: loongarch_target_feature) , & []) , ("f" , Stable , & []) , ("frecipe" , Stable , & []) , ("lam-bh" , Unstable (sym :: loongarch_target_feature) , & []) , ("lamcas" , Unstable (sym :: loongarch_target_feature) , & []) , ("lasx" , Stable , & ["lsx"]) , ("lbt" , Stable , & []) , ("ld-seq-sa" , Unstable (sym :: loongarch_target_feature) , & []) , ("lsx" , Stable , & ["d"]) , ("lvz" , Stable , & []) , ("relax" , Unstable (sym :: loongarch_target_feature) , & []) , ("scq" , Unstable (sym :: loongarch_target_feature) , & []) , ("ual" , Unstable (sym :: loongarch_target_feature) , & []) ,] ;
    };
}

LOONGARCH_FEATURES!()