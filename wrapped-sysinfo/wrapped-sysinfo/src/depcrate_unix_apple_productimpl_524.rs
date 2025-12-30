// Generated macro for impl_524 (impl)
macro_rules! Depcrate_unix_apple_productimpl_524 {
() => {
// Module: crate::unix::apple::product
// Provides: {"impl_524"}
// Dependencies: {}
impl ProductInner { pub (crate) fn family () -> Option < String > { Some (get_sysctl_str (b"hw.model\0")) } pub (crate) fn name () -> Option < String > { Self :: family () . or_else (| | { cfg_if ! { if # [cfg (all (target_os = "macos" , not (feature = "apple-sandbox")))] { get_io_platform_property ("product-name") } else { None } } }) } pub (crate) fn serial_number () -> Option < String > { cfg_if ! { if # [cfg (all (target_os = "macos" , not (feature = "apple-sandbox")))] { use objc2_io_kit :: kIOPlatformSerialNumberKey ; get_io_platform_property (unsafe { std :: str :: from_utf8_unchecked (kIOPlatformSerialNumberKey . to_bytes ()) }) } else { None } } } pub (crate) fn stock_keeping_unit () -> Option < String > { None } pub (crate) fn uuid () -> Option < String > { cfg_if ! { if # [cfg (all (target_os = "macos" , not (feature = "apple-sandbox")))] { use objc2_io_kit :: kIOPlatformUUIDKey ; get_io_platform_property (unsafe { std :: str :: from_utf8_unchecked (kIOPlatformUUIDKey . to_bytes ()) }) } else { None } } } pub (crate) fn version () -> Option < String > { cfg_if ! { if # [cfg (all (target_os = "macos" , not (feature = "apple-sandbox")))] { get_io_platform_property ("version") } else { None } } } pub (crate) fn vendor_name () -> Option < String > { crate :: Motherboard :: new () . and_then (| m | m . vendor_name ()) } }
};
}
