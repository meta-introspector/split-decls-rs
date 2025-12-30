// Generated macro for POWERPC_FEATURES (static)
macro_rules! Depcrate_target_featuresPOWERPC_FEATURES {
() => {
// Module: crate::target_features
// Provides: {"POWERPC_FEATURES"}
// Dependencies: {}
static POWERPC_FEATURES : & [(& str , Stability , ImpliedFeatures)] = & [("altivec" , Unstable (sym :: powerpc_target_feature) , & []) , ("msync" , Unstable (sym :: powerpc_target_feature) , & []) , ("partword-atomics" , Unstable (sym :: powerpc_target_feature) , & []) , ("power8-altivec" , Unstable (sym :: powerpc_target_feature) , & ["altivec"]) , ("power8-crypto" , Unstable (sym :: powerpc_target_feature) , & ["power8-altivec"]) , ("power8-vector" , Unstable (sym :: powerpc_target_feature) , & ["vsx" , "power8-altivec"]) , ("power9-altivec" , Unstable (sym :: powerpc_target_feature) , & ["power8-altivec"]) , ("power9-vector" , Unstable (sym :: powerpc_target_feature) , & ["power8-vector" , "power9-altivec"]) , ("power10-vector" , Unstable (sym :: powerpc_target_feature) , & ["power9-vector"]) , ("quadword-atomics" , Unstable (sym :: powerpc_target_feature) , & []) , ("vsx" , Unstable (sym :: powerpc_target_feature) , & ["altivec"]) ,] ;
};
}
