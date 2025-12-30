// Generated macro for tests (module)
macro_rules! Depcrate_filters_pathtests {
() => {
// Module: crate::filters::path
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_path_exact_size () { use std :: mem :: { size_of , size_of_val } ; assert_eq ! (size_of_val (& path ("hello")) , size_of ::<& str > () , "exact(&str) is size of &str") ; assert_eq ! (size_of_val (& path (String :: from ("world"))) , size_of ::< String > () , "exact(String) is size of String") ; assert_eq ! (size_of_val (& path ! ("zst")) , size_of ::< () > () , "path!(&str) is ZST") ; } }
};
}
