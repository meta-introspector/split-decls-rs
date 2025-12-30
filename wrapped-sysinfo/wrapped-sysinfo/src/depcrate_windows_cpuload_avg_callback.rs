// Generated macro for load_avg_callback (function)
macro_rules! Depcrate_windows_cpuload_avg_callback {
() => {
// Module: crate::windows::cpu
// Provides: {"load_avg_callback"}
// Dependencies: {}
unsafe extern "system" fn load_avg_callback (counter : * mut c_void , _ : bool) { let mut display_value = mem :: MaybeUninit :: < PDH_FMT_COUNTERVALUE > :: uninit () ; unsafe { if PdhGetFormattedCounterValue (PDH_HCOUNTER (counter) , PDH_FMT_DOUBLE , None , display_value . as_mut_ptr () ,) != ERROR_SUCCESS . 0 { return ; } let display_value = display_value . assume_init () ; if let Ok (mut avg) = load_avg () . lock () && let Some (avg) = avg . deref_mut () { let current_load = display_value . Anonymous . doubleValue ; avg . one = avg . one * LOADAVG_FACTOR_1F + current_load * (1.0 - LOADAVG_FACTOR_1F) ; avg . five = avg . five * LOADAVG_FACTOR_5F + current_load * (1.0 - LOADAVG_FACTOR_5F) ; avg . fifteen = avg . fifteen * LOADAVG_FACTOR_15F + current_load * (1.0 - LOADAVG_FACTOR_15F) ; } } }
};
}
