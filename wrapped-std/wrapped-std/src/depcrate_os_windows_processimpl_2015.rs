// Generated macro for impl_2015 (impl)
macro_rules! Depcrate_os_windows_processimpl_2015 {
() => {
// Module: crate::os::windows::process
// Provides: {"impl_2015"}
// Dependencies: {}
# [unstable (feature = "windows_process_extensions_raw_attribute" , issue = "114854")] impl < 'a > Drop for ProcThreadAttributeList < 'a > { # [doc = " Deletes the attribute list."] # [doc = ""] # [doc = " This method calls [`DeleteProcThreadAttributeList`][1] to delete the"] # [doc = " underlying attribute list."] # [doc = ""] # [doc = " [1]: <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-deleteprocthreadattributelist>"] fn drop (& mut self) { let lp_attribute_list = self . attribute_list . as_mut_ptr () . cast :: < c_void > () ; unsafe { sys :: c :: DeleteProcThreadAttributeList (lp_attribute_list) } } }
};
}
