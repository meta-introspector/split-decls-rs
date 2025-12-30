// Generated macro for impl_798 (impl)
macro_rules! Depcrate_unix_linux_motherboardimpl_798 {
() => {
// Module: crate::unix::linux::motherboard
// Provides: {"impl_798"}
// Dependencies: {}
impl MotherboardInner { pub (crate) fn new () -> Option < Self > { Some (Self) } pub (crate) fn asset_tag (& self) -> Option < String > { read_to_string ("/sys/devices/virtual/dmi/id/board_asset_tag") . ok () . map (| s | s . trim () . to_owned ()) } pub (crate) fn name (& self) -> Option < String > { read_to_string ("/sys/devices/virtual/dmi/id/board_name") . ok () . or_else (| | { read_to_string ("/proc/device-tree/board") . ok () . or_else (| | Some (parse_device_tree_compatible () ? . 1)) }) . map (| s | s . trim () . to_owned ()) } pub (crate) fn vendor_name (& self) -> Option < String > { read_to_string ("/sys/devices/virtual/dmi/id/board_vendor") . ok () . or_else (| | Some (parse_device_tree_compatible () ? . 0)) . map (| s | s . trim () . to_owned ()) } pub (crate) fn version (& self) -> Option < String > { read_to_string ("/sys/devices/virtual/dmi/id/board_version") . ok () . map (| s | s . trim () . to_owned ()) } pub (crate) fn serial_number (& self) -> Option < String > { read_to_string ("/sys/devices/virtual/dmi/id/board_serial") . ok () . map (| s | s . trim () . to_owned ()) } }
};
}
