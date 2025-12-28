macro_rules! is_enabled_avx2_and_bmi2 {
    () => {
        # [inline (always)] pub fn is_enabled_avx2_and_bmi2 () -> bool { # [cfg (any (target_arch = "x86_64" , target_arch = "x86"))] { # [cfg (all (target_feature = "avx2" , target_feature = "bmi1" , target_feature = "bmi2"))] return true ; # [cfg (feature = "std")] { use std :: sync :: atomic :: { AtomicU32 , Ordering } ; static CACHE : AtomicU32 = AtomicU32 :: new (2) ; return match CACHE . load (Ordering :: Relaxed) { 0 => false , 1 => true , _ => { let detected = std :: is_x86_feature_detected ! ("avx2") && std :: is_x86_feature_detected ! ("bmi1") && std :: is_x86_feature_detected ! ("bmi2") ; CACHE . store (u32 :: from (detected) , Ordering :: Relaxed) ; detected } } ; } } false }
    };
}

is_enabled_avx2_and_bmi2!()