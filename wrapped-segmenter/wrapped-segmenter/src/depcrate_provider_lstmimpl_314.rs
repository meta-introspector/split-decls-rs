// Generated macro for impl_314 (impl)
macro_rules! Depcrate_provider_lstmimpl_314 {
() => {
// Module: crate::provider::lstm
// Provides: {"impl_314"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl databake :: Bake for LstmDataFloat32 < '_ > { fn bake (& self , env : & databake :: CrateEnv) -> databake :: TokenStream { let model = self . model . bake (env) ; let dic = self . dic . bake (env) ; let embedding = self . embedding . bake (env) ; let fw_w = self . fw_w . bake (env) ; let fw_u = self . fw_u . bake (env) ; let fw_b = self . fw_b . bake (env) ; let bw_w = self . bw_w . bake (env) ; let bw_u = self . bw_u . bake (env) ; let bw_b = self . bw_b . bake (env) ; let time_w = self . time_w . bake (env) ; let time_b = self . time_b . bake (env) ; databake :: quote ! { icu_segmenter :: provider :: LstmDataFloat32 :: from_parts_unchecked (# model , # dic , # embedding , # fw_w , # fw_u , # fw_b , # bw_w , # bw_u , # bw_b , # time_w , # time_b ,) } } }
};
}
