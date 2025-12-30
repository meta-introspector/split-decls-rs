// Generated macro for macro_10858 (macro)
macro_rules! Depcrate_shared_usbioctlmacro_10858 {
() => {
// Module: crate::shared::usbioctl
// Provides: {"macro_10858"}
// Dependencies: {}
STRUCT ! { # [repr (packed)] struct USB_DEVICE_PERFORMANCE_INFO { BulkBytes : ULONG , ControlDataBytes : ULONG , IsoBytes : ULONG , InterruptBytes : ULONG , BulkUrbCount : ULONG , ControlUrbCount : ULONG , IsoUrbCount : ULONG , InterruptUrbCount : ULONG , AllocedInterrupt : [ULONG ; 6] , AllocedIso : ULONG , Total32secBandwidth : ULONG , TotalTtBandwidth : ULONG , DeviceDescription : [WCHAR ; 60] , DeviceSpeed : USB_DEVICE_SPEED , TotalIsoLatency : ULONG , DroppedIsoPackets : ULONG , TransferErrors : ULONG , PciInterruptCount : ULONG , HcIdleState : ULONG , HcAsyncIdleState : ULONG , HcAsyncCacheFlushCount : ULONG , HcPeriodicIdleState : ULONG , HcPeriodicCacheFlushCount : ULONG , } }
};
}
