// Generated macro for impl_120 (impl)
macro_rules! Depcrateimpl_120 {
() => {
// Module: crate
// Provides: {"impl_120"}
// Dependencies: {}
impl fmt :: Debug for WalkDirOptions { fn fmt (& self , f : & mut fmt :: Formatter < '_ > ,) -> result :: Result < () , fmt :: Error > { let sorter_str = if self . sorter . is_some () { "Some(...)" } else { "None" } ; f . debug_struct ("WalkDirOptions") . field ("follow_links" , & self . follow_links) . field ("follow_root_link" , & self . follow_root_links) . field ("max_open" , & self . max_open) . field ("min_depth" , & self . min_depth) . field ("max_depth" , & self . max_depth) . field ("sorter" , & sorter_str) . field ("contents_first" , & self . contents_first) . field ("same_file_system" , & self . same_file_system) . finish () } }
};
}
