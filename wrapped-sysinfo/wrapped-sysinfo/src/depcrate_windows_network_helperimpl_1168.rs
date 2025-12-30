// Generated macro for impl_1168 (impl)
macro_rules! Depcrate_windows_network_helperimpl_1168 {
() => {
// Module: crate::windows::network_helper
// Provides: {"impl_1168"}
// Dependencies: {}
impl InterfaceAddressIterator { fn new () -> Self { Self { buf : null_mut () , adapter : null_mut () , } } unsafe fn realloc (mut self , size : libc :: size_t) -> Result < Self , String > { let new_buf = unsafe { libc :: realloc (self . buf as _ , size) as * mut IP_ADAPTER_ADDRESSES_LH } ; if new_buf . is_null () { Err ("failed to allocate memory for IP_ADAPTER_ADDRESSES" . to_string ()) } else { self . buf = new_buf ; self . adapter = new_buf ; Ok (self) } } }
};
}
