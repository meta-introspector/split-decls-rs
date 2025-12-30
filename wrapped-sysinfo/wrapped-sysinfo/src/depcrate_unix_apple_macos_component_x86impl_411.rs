// Generated macro for impl_411 (impl)
macro_rules! Depcrate_unix_apple_macos_component_x86impl_411 {
() => {
// Module: crate::unix::apple::macos::component::x86
// Provides: {"impl_411"}
// Dependencies: {}
impl IoService { fn new (obj : io_connect_t) -> Option < Self > { if obj == 0 { None } else { Some (Self (obj)) } } pub (crate) fn inner (& self) -> io_connect_t { self . 0 } pub (crate) fn new_connection () -> Option < Self > { let mut iterator : io_iterator_t = 0 ; unsafe { let Some (matching) = IOServiceMatching (c"AppleSMC" . as_ptr () as * const i8) else { sysinfo_debug ! ("IOServiceMatching call failed, `AppleSMC` not found") ; return None ; } ; let matching = CFRetained :: < CFDictionary > :: from (& matching) ; let result = IOServiceGetMatchingServices (kIOMainPortDefault , Some (matching) , & mut iterator) ; if result != kIOReturnSuccess { sysinfo_debug ! ("Error: IOServiceGetMatchingServices() = {}" , result) ; return None ; } let iterator = match IOReleaser :: new (iterator) { Some (i) => i , None => { sysinfo_debug ! ("Error: IOServiceGetMatchingServices() succeeded but returned invalid descriptor") ; return None ; } } ; let device = match IOReleaser :: new (IOIteratorNext (iterator . inner ())) { Some (d) => d , None => { sysinfo_debug ! ("Error: no SMC found") ; return None ; } } ; let mut conn = 0 ; let result = IOServiceOpen (device . inner () , # [allow (deprecated)] libc :: mach_task_self () , 0 , & mut conn ,) ; if result != kIOReturnSuccess { sysinfo_debug ! ("Error: IOServiceOpen() = {}" , result) ; return None ; } let conn = IoService :: new (conn) ; if conn . is_none () { sysinfo_debug ! ("Error: IOServiceOpen() succeeded but returned invalid descriptor...") ; } conn } } }
};
}
