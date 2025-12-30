// Generated macro for iterate_service_tree (function)
macro_rules! Depcrate_unix_apple_macos_diskiterate_service_tree {
() => {
// Module: crate::unix::apple::macos::disk
// Provides: {"iterate_service_tree"}
// Dependencies: {}
fn iterate_service_tree < T , F > (bsd_name : & [u8] , key : & CFString , eval : F) -> Option < T > where F : Fn (io_registry_entry_t , & CFDictionary) -> Option < T > , { let matching = unsafe { IOBSDNameMatching (kIOMainPortDefault , 0 , bsd_name . as_ptr () . cast ()) } ? ; let matching = CFRetained :: < CFDictionary > :: from (& matching) ; let mut service_iterator : io_iterator_t = 0 ; if unsafe { IOServiceGetMatchingServices (kIOMainPortDefault , Some (matching) , & mut service_iterator) } != libc :: KERN_SUCCESS { return None ; } let service_iterator = unsafe { IOReleaser :: new_unchecked (service_iterator) } ; let mut parent_entry : io_registry_entry_t = 0 ; while let Some (mut current_service_entry) = IOReleaser :: new (IOIteratorNext (service_iterator . inner ())) { loop { if unsafe { IORegistryEntryGetParentEntry (current_service_entry . inner () , kIOServicePlane . as_ptr () . cast_mut () . cast () , & mut parent_entry ,) } != libc :: KERN_SUCCESS { break ; } current_service_entry = match IOReleaser :: new (parent_entry) { Some (service) => service , None => break , } ; let properties_result = unsafe { IORegistryEntryCreateCFProperty (current_service_entry . inner () , Some (key) , kCFAllocatorDefault , 0 ,) } ; if let Some (properties) = properties_result && let Ok (properties) = properties . downcast :: < CFDictionary > () && let Some (result) = eval (parent_entry , & properties) { return Some (result) ; } } } None }
};
}
