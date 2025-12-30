// Generated macro for impl_492 (impl)
macro_rules! Depcrate_unix_apple_motherboardimpl_492 {
() => {
// Module: crate::unix::apple::motherboard
// Provides: {"impl_492"}
// Dependencies: {}
impl MotherboardInner { pub (crate) fn new () -> Option < Self > { Some (Self) } pub (crate) fn name (& self) -> Option < String > { cfg_if ! { if # [cfg (all (target_os = "macos" , not (feature = "apple-sandbox")))] { get_io_platform_property ("board-id") } else { None } } } pub (crate) fn vendor_name (& self) -> Option < String > { cfg_if ! { if # [cfg (all (target_os = "macos" , not (feature = "apple-sandbox")))] { get_io_platform_property ("manufacturer") } else { None } } } pub (crate) fn version (& self) -> Option < String > { cfg_if ! { if # [cfg (all (target_os = "macos" , not (feature = "apple-sandbox")))] { get_io_platform_property ("version") } else { None } } } pub (crate) fn serial_number (& self) -> Option < String > { cfg_if ! { if # [cfg (all (target_os = "macos" , not (feature = "apple-sandbox")))] { get_io_platform_property ("IOPlatformSerialNumber") } else { None } } } pub (crate) fn asset_tag (& self) -> Option < String > { None } }
};
}
