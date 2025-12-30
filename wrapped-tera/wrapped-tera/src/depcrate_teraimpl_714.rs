// Generated macro for impl_714 (impl)
macro_rules! Depcrate_teraimpl_714 {
() => {
// Module: crate::tera
// Provides: {"impl_714"}
// Dependencies: {}
impl Default for Tera { fn default () -> Tera { let mut tera = Tera { glob : None , templates : HashMap :: new () , filters : HashMap :: new () , testers : HashMap :: new () , functions : HashMap :: new () , autoescape_suffixes : vec ! [".html" , ".htm" , ".xml"] , escape_fn : escape_html , } ; tera . register_tera_filters () ; tera . register_tera_testers () ; tera . register_tera_functions () ; tera } }
};
}
