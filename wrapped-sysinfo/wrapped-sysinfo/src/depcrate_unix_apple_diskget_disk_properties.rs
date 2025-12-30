// Generated macro for get_disk_properties (function)
macro_rules! Depcrate_unix_apple_diskget_disk_properties {
() => {
// Module: crate::unix::apple::disk
// Provides: {"get_disk_properties"}
// Dependencies: {}
fn get_disk_properties (volume_url : & CFURL , requested_properties : & CFArray ,) -> Option < CFRetained < CFDictionary > > { unsafe { volume_url . resource_properties_for_keys (Some (requested_properties) , ptr :: null_mut ()) } }
};
}
