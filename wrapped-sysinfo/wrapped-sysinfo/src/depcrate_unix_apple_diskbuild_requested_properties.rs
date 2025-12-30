// Generated macro for build_requested_properties (function)
macro_rules! Depcrate_unix_apple_diskbuild_requested_properties {
() => {
// Module: crate::unix::apple::disk
// Provides: {"build_requested_properties"}
// Dependencies: {}
unsafe fn build_requested_properties (properties : & [Option < & CFString >] ,) -> Option < CFRetained < CFArray > > { unsafe { CFArray :: new (None , properties . as_ptr () as * mut * const c_void , properties . len () as _ , & kCFTypeArrayCallBacks ,) } }
};
}
