// Generated macro for PreprocessorCacheEntry (struct)
macro_rules! Depcrate_compiler_preprocessor_cachePreprocessorCacheEntry {
() => {
// Module: crate::compiler::preprocessor_cache
// Provides: {"PreprocessorCacheEntry"}
// Dependencies: {}
# [derive (Deserialize , Serialize , Debug , Default , PartialEq , Eq)] pub struct PreprocessorCacheEntry { # [doc = " A counter of the overall number of [`IncludeEntry`] in this"] # [doc = " preprocessor cache entry, as an optimization when checking"] # [doc = " we're not ballooning in size."] number_of_entries : usize , # [doc = " The digest of a result is computed by hashing the output of the"] # [doc = " C preprocessor. Entries correspond to the included files during the"] # [doc = " preprocessing step."] results : BTreeMap < String , Vec < IncludeEntry > > , }
};
}
