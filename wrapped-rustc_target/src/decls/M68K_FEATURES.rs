macro_rules! deps {
    () => {
        Stability!();
        ImpliedFeatures!();
    };
}

macro_rules! M68K_FEATURES {
    () => {
        deps!();
        static M68K_FEATURES : & [(& str , Stability , ImpliedFeatures)] = & [("isa-68000" , Unstable (sym :: m68k_target_feature) , & []) , ("isa-68010" , Unstable (sym :: m68k_target_feature) , & ["isa-68000"]) , ("isa-68020" , Unstable (sym :: m68k_target_feature) , & ["isa-68010"]) , ("isa-68030" , Unstable (sym :: m68k_target_feature) , & ["isa-68020"]) , ("isa-68040" , Unstable (sym :: m68k_target_feature) , & ["isa-68030" , "isa-68882"]) , ("isa-68060" , Unstable (sym :: m68k_target_feature) , & ["isa-68040"]) , ("isa-68881" , Unstable (sym :: m68k_target_feature) , & []) , ("isa-68882" , Unstable (sym :: m68k_target_feature) , & ["isa-68881"]) ,] ;
    };
}

M68K_FEATURES!()