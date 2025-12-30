// Generated macro for impl_475 (impl)
macro_rules! Depcrate_unix_apple_macos_systemimpl_475 {
() => {
// Module: crate::unix::apple::macos::system
// Provides: {"impl_475"}
// Dependencies: {}
impl Drop for ProcessorCpuLoadInfo { fn drop (& mut self) { unsafe { munmap (self . cpu_load as _ , vm_page_size) ; } } }
};
}
