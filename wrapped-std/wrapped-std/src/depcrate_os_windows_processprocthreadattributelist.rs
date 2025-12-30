// Generated macro for ProcThreadAttributeList (struct)
macro_rules! Depcrate_os_windows_processProcThreadAttributeList {
() => {
// Module: crate::os::windows::process
// Provides: {"ProcThreadAttributeList"}
// Dependencies: {}
# [doc = " A wrapper around windows [`ProcThreadAttributeList`][1]."] # [doc = ""] # [doc = " [1]: <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-initializeprocthreadattributelist>"] # [derive (Debug)] # [unstable (feature = "windows_process_extensions_raw_attribute" , issue = "114854")] pub struct ProcThreadAttributeList < 'a > { attribute_list : Box < [MaybeUninit < u8 >] > , _lifetime_marker : marker :: PhantomData < & 'a () > , }
};
}
