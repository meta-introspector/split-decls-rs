// Generated macro for delay_load (function)
macro_rules! Depcratedelay_load {
() => {
// Module: crate
// Provides: {"delay_load"}
// Dependencies: {}
unsafe fn delay_load < T > (library : bindings :: PCSTR , function : bindings :: PCSTR) -> Option < T > { unsafe { let library = LoadLibraryExA (library , std :: ptr :: null_mut () , LOAD_LIBRARY_SEARCH_DEFAULT_DIRS ,) ; if library . is_null () { return None ; } ; let address = GetProcAddress (library , function) ; if address . is_some () { return Some (std :: mem :: transmute_copy (& address)) ; } None } }
};
}
