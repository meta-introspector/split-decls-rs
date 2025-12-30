// Generated macro for impl_129 (impl)
macro_rules! Depcrate_common_diskimpl_129 {
() => {
// Module: crate::common::disk
// Provides: {"impl_129"}
// Dependencies: {}
impl DiskRefreshKind { # [doc = " Creates a new `DiskRefreshKind` with every refresh set to false."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::DiskRefreshKind;"] # [doc = ""] # [doc = " let r = DiskRefreshKind::nothing();"] # [doc = ""] # [doc = " assert_eq!(r.kind(), false);"] # [doc = " assert_eq!(r.storage(), false);"] # [doc = " assert_eq!(r.io_usage(), false);"] # [doc = " ```"] pub fn nothing () -> Self { Self :: default () } # [doc = " Creates a new `DiskRefreshKind` with every refresh set to true."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::DiskRefreshKind;"] # [doc = ""] # [doc = " let r = DiskRefreshKind::everything();"] # [doc = ""] # [doc = " assert_eq!(r.kind(), true);"] # [doc = " assert_eq!(r.storage(), true);"] # [doc = " assert_eq!(r.io_usage(), true);"] # [doc = " ```"] pub fn everything () -> Self { Self { kind : true , storage : true , io_usage : true , } } impl_get_set ! (DiskRefreshKind , kind , with_kind , without_kind) ; impl_get_set ! (DiskRefreshKind , storage , with_storage , without_storage) ; impl_get_set ! (DiskRefreshKind , io_usage , with_io_usage , without_io_usage) ; }
};
}
