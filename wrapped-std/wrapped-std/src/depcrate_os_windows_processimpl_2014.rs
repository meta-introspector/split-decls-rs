// Generated macro for impl_2014 (impl)
macro_rules! Depcrate_os_windows_processimpl_2014 {
() => {
// Module: crate::os::windows::process
// Provides: {"impl_2014"}
// Dependencies: {}
# [unstable (feature = "windows_process_extensions_raw_attribute" , issue = "114854")] impl < 'a > ProcThreadAttributeList < 'a > { # [doc = " Creates a new builder for constructing a [`ProcThreadAttributeList`]."] pub fn build () -> ProcThreadAttributeListBuilder < 'a > { ProcThreadAttributeListBuilder :: new () } # [doc = " Returns a pointer to the underling attribute list."] # [doc (hidden)] pub fn as_ptr (& self) -> * const MaybeUninit < u8 > { self . attribute_list . as_ptr () } }
};
}
