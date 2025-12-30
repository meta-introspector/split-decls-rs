// Generated macro for get_disk_type (function)
macro_rules! Depcrate_unix_apple_macos_diskget_disk_type {
() => {
// Module: crate::unix::apple::macos::disk
// Provides: {"get_disk_type"}
// Dependencies: {}
pub (crate) fn get_disk_type (bsd_name : & [u8]) -> Option < DiskKind > { let characteristics_string = CFString :: from_static_str (ffi :: kIOPropertyDeviceCharacteristicsKey) ; iterate_service_tree (bsd_name , & characteristics_string , | _ , properties | { let medium = unsafe { super :: disk :: get_str_value (properties , Some (& CFString :: from_static_str (ffi :: kIOPropertyMediumTypeKey)) ,) } ? ; match medium . as_str () { _ if medium == ffi :: kIOPropertyMediumTypeSolidStateKey => Some (DiskKind :: SSD) , _ if medium == ffi :: kIOPropertyMediumTypeRotationalKey => Some (DiskKind :: HDD) , _ => Some (DiskKind :: Unknown (- 1)) , } }) }
};
}
