macro_rules! deps {
    () => {
        ImpliedFeatures!();
        Stability!();
    };
}

macro_rules! MIPS_FEATURES {
    () => {
        deps!();
        const MIPS_FEATURES : & [(& str , Stability , ImpliedFeatures)] = & [("fp64" , Unstable (sym :: mips_target_feature) , & []) , ("msa" , Unstable (sym :: mips_target_feature) , & []) , ("virt" , Unstable (sym :: mips_target_feature) , & []) ,] ;
    };
}

MIPS_FEATURES!()