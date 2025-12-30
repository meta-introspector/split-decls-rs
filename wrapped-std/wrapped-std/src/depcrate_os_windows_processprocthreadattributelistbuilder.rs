// Generated macro for ProcThreadAttributeListBuilder (struct)
macro_rules! Depcrate_os_windows_processProcThreadAttributeListBuilder {
() => {
// Module: crate::os::windows::process
// Provides: {"ProcThreadAttributeListBuilder"}
// Dependencies: {}
# [doc = " Builder for constructing a [`ProcThreadAttributeList`]."] # [derive (Clone , Debug)] # [unstable (feature = "windows_process_extensions_raw_attribute" , issue = "114854")] pub struct ProcThreadAttributeListBuilder < 'a > { attributes : alloc :: collections :: BTreeMap < usize , ProcThreadAttributeValue > , _lifetime_marker : marker :: PhantomData < & 'a () > , }
};
}
