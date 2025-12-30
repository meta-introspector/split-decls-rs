// Generated macro for collect_lang_features (function)
macro_rules! Depcrate_featurescollect_lang_features {
() => {
// Module: crate::features
// Provides: {"collect_lang_features"}
// Dependencies: {}
pub fn collect_lang_features (base_compiler_path : & Path , bad : & mut bool) -> Features { let mut features = Features :: new () ; collect_lang_features_in (& mut features , base_compiler_path , "accepted.rs" , bad) ; collect_lang_features_in (& mut features , base_compiler_path , "removed.rs" , bad) ; collect_lang_features_in (& mut features , base_compiler_path , "unstable.rs" , bad) ; features }
};
}
