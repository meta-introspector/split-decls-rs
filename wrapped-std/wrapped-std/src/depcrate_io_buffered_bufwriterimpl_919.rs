// Generated macro for impl_919 (impl)
macro_rules! Depcrate_io_buffered_bufwriterimpl_919 {
() => {
// Module: crate::io::buffered::bufwriter
// Provides: {"impl_919"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < W : ? Sized + Write > Drop for BufWriter < W > { fn drop (& mut self) { if ! self . panicked { let _r = self . flush_buf () ; } } }
};
}
