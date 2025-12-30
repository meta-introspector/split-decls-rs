// Generated macro for impl_125 (impl)
macro_rules! Depcrate_androidimpl_125 {
() => {
// Module: crate::android
// Provides: {"impl_125"}
// Dependencies: {}
impl CachedClass { # [doc = " Creates a lazily initialized class reference to the class with `name`."] pub (super) const fn new (name : & 'static str) -> Self { Self { name , class : OnceCell :: new () , } } # [doc = " Gets the cached class reference, loaded on first use"] pub (super) fn get (& self , cx : & mut LocalContext) -> Result < & JClass < '_ > , Error > { let class = self . class . get_or_try_init (| | -> Result < _ , Error > { let class = cx . load_class (self . name) ? ; Ok (cx . env . new_global_ref (class) ?) }) ? ; Ok (class . as_obj () . into ()) } }
};
}
