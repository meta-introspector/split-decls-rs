// Generated macro for impl_4847 (impl)
macro_rules! Depcrate_features_gen_NativeOsFileWriteAtomicOptionsimpl_4847 {
() => {
// Module: crate::features::gen_NativeOsFileWriteAtomicOptions
// Provides: {"impl_4847"}
// Dependencies: {}
impl NativeOsFileWriteAtomicOptions { # [doc = "Construct a new `NativeOsFileWriteAtomicOptions`."] # [doc = ""] # [doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"] pub fn new () -> Self { # [allow (unused_mut)] let mut ret : Self = :: wasm_bindgen :: JsCast :: unchecked_into (:: js_sys :: Object :: new ()) ; ret } # [deprecated = "Use `set_backup_to()` instead."] pub fn backup_to (& mut self , val : Option < & str >) -> & mut Self { self . set_backup_to (val) ; self } # [deprecated = "Use `set_bytes()` instead."] pub fn bytes (& mut self , val : Option < f64 >) -> & mut Self { self . set_bytes (val) ; self } # [deprecated = "Use `set_flush()` instead."] pub fn flush (& mut self , val : bool) -> & mut Self { self . set_flush (val) ; self } # [deprecated = "Use `set_no_overwrite()` instead."] pub fn no_overwrite (& mut self , val : bool) -> & mut Self { self . set_no_overwrite (val) ; self } # [deprecated = "Use `set_tmp_path()` instead."] pub fn tmp_path (& mut self , val : Option < & str >) -> & mut Self { self . set_tmp_path (val) ; self } }
};
}
