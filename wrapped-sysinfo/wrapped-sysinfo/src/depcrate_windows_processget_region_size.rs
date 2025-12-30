// Generated macro for get_region_size (function)
macro_rules! Depcrate_windows_processget_region_size {
() => {
// Module: crate::windows::process
// Provides: {"get_region_size"}
// Dependencies: {}
unsafe fn get_region_size (handle : HANDLE , ptr : * const c_void) -> Result < usize , & 'static str > { let mut meminfo = MaybeUninit :: < MEMORY_BASIC_INFORMATION > :: uninit () ; unsafe { if VirtualQueryEx (handle , Some (ptr) , meminfo . as_mut_ptr () . cast () , size_of :: < MEMORY_BASIC_INFORMATION > () ,) == 0 { return Err ("Unable to read process memory information") ; } let meminfo = meminfo . assume_init () ; Ok ((meminfo . RegionSize as isize - ptr . offset_from (meminfo . BaseAddress)) as usize) } }
};
}
