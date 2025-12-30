// Generated macro for impl_36 (impl)
macro_rules! Depcrate_loaderimpl_36 {
() => {
// Module: crate::loader
// Provides: {"impl_36"}
// Dependencies: {}
impl fmt :: Debug for Message { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Message :: Loaded { files } => { f . debug_struct ("Loaded") . field ("n_files" , & files . len ()) . finish () } Message :: Changed { files } => { f . debug_struct ("Changed") . field ("n_files" , & files . len ()) . finish () } Message :: Progress { n_total , n_done , dir , config_version } => f . debug_struct ("Progress") . field ("n_total" , n_total) . field ("n_done" , n_done) . field ("dir" , dir) . field ("config_version" , config_version) . finish () , } } }
};
}
