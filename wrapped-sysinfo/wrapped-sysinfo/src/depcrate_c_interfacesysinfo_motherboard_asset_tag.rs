// Generated macro for sysinfo_motherboard_asset_tag (function)
macro_rules! Depcrate_c_interfacesysinfo_motherboard_asset_tag {
() => {
// Module: crate::c_interface
// Provides: {"sysinfo_motherboard_asset_tag"}
// Dependencies: {}
# [doc = " Equivalent of [`Motherboard::asset_tag()`]."] # [unsafe (no_mangle)] pub extern "C" fn sysinfo_motherboard_asset_tag () -> RString { if let Some (c) = Motherboard :: new () . and_then (| m | m . asset_tag ()) . and_then (| c | CString :: new (c) . ok ()) { c . into_raw () as _ } else { std :: ptr :: null () } }
};
}
