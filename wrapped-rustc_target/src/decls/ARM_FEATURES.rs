macro_rules! deps {
    () => {
        Stability!();
        ABI!();
        ImpliedFeatures!();
    };
}

macro_rules! ARM_FEATURES {
    () => {
        deps!();
        static ARM_FEATURES : & [(& str , Stability , ImpliedFeatures)] = & [("aclass" , Unstable (sym :: arm_target_feature) , & []) , ("aes" , Unstable (sym :: arm_target_feature) , & ["neon"]) , ("atomics-32" , Stability :: Forbidden { reason : "unsound because it changes the ABI of atomic operations" } , & [] ,) , ("crc" , Unstable (sym :: arm_target_feature) , & []) , ("d32" , Unstable (sym :: arm_target_feature) , & []) , ("dotprod" , Unstable (sym :: arm_target_feature) , & ["neon"]) , ("dsp" , Unstable (sym :: arm_target_feature) , & []) , ("fp-armv8" , Unstable (sym :: arm_target_feature) , & ["vfp4"]) , ("fp16" , Unstable (sym :: arm_target_feature) , & ["neon"]) , ("fpregs" , Unstable (sym :: arm_target_feature) , & []) , ("i8mm" , Unstable (sym :: arm_target_feature) , & ["neon"]) , ("mclass" , Unstable (sym :: arm_target_feature) , & []) , ("neon" , Unstable (sym :: arm_target_feature) , & ["vfp3"]) , ("rclass" , Unstable (sym :: arm_target_feature) , & []) , ("sha2" , Unstable (sym :: arm_target_feature) , & ["neon"]) , ("soft-float" , Unstable (sym :: arm_target_feature) , & []) , ("thumb-mode" , Unstable (sym :: arm_target_feature) , & []) , ("thumb2" , Unstable (sym :: arm_target_feature) , & []) , ("trustzone" , Unstable (sym :: arm_target_feature) , & []) , ("v5te" , Unstable (sym :: arm_target_feature) , & []) , ("v6" , Unstable (sym :: arm_target_feature) , & ["v5te"]) , ("v6k" , Unstable (sym :: arm_target_feature) , & ["v6"]) , ("v6t2" , Unstable (sym :: arm_target_feature) , & ["v6k" , "thumb2"]) , ("v7" , Unstable (sym :: arm_target_feature) , & ["v6t2"]) , ("v8" , Unstable (sym :: arm_target_feature) , & ["v7"]) , ("vfp2" , Unstable (sym :: arm_target_feature) , & []) , ("vfp3" , Unstable (sym :: arm_target_feature) , & ["vfp2" , "d32"]) , ("vfp4" , Unstable (sym :: arm_target_feature) , & ["vfp3"]) , ("virtualization" , Unstable (sym :: arm_target_feature) , & []) ,] ;
    };
}

ARM_FEATURES!()