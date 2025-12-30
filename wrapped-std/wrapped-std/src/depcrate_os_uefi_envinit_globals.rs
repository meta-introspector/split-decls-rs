// Generated macro for init_globals (function)
macro_rules! Depcrate_os_uefi_envinit_globals {
() => {
// Module: crate::os::uefi::env
// Provides: {"init_globals"}
// Dependencies: {}
# [doc = " Initializes the global System Table and Image Handle pointers."] # [doc = ""] # [doc = " The standard library requires access to the UEFI System Table and the Application Image Handle"] # [doc = " to operate. Those are provided to UEFI Applications via their application entry point. By"] # [doc = " calling `init_globals()`, those pointers are retained by the standard library for future use."] # [doc = " Thus this function must be called before any of the standard library services are used."] # [doc = ""] # [doc = " The pointers are never exposed to any entity outside of this application and it is guaranteed"] # [doc = " that, once the application exited, these pointers are never dereferenced again."] # [doc = ""] # [doc = " Callers are required to ensure the pointers are valid for the entire lifetime of this"] # [doc = " application. In particular, UEFI Boot Services must not be exited while an application with the"] # [doc = " standard library is loaded."] # [doc = ""] # [doc = " # SAFETY"] # [doc = " Calling this function more than once will panic."] pub (crate) unsafe fn init_globals (handle : NonNull < c_void > , system_table : NonNull < c_void >) { IMAGE_HANDLE . compare_exchange (crate :: ptr :: null_mut () , handle . as_ptr () , Ordering :: Release , Ordering :: Acquire ,) . unwrap () ; SYSTEM_TABLE . compare_exchange (crate :: ptr :: null_mut () , system_table . as_ptr () , Ordering :: Release , Ordering :: Acquire ,) . unwrap () ; BOOT_SERVICES_FLAG . store (true , Ordering :: Release) }
};
}
