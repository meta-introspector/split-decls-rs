// Generated macro for macro_5992 (macro)
macro_rules! Depcrate_shared_ntddscsimacro_5992 {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"macro_5992"}
// Dependencies: {}
STRUCT ! { struct STORAGE_FIRMWARE_INFO_V2 { Version : ULONG , Size : ULONG , UpgradeSupport : BOOLEAN , SlotCount : UCHAR , ActiveSlot : UCHAR , PendingActivateSlot : UCHAR , FirmwareShared : BOOLEAN , Reserved : [UCHAR ; 3] , ImagePayloadAlignment : ULONG , ImagePayloadMaxSize : ULONG , Slot : [STORAGE_FIRMWARE_SLOT_INFO_V2 ; 0] , } }
};
}
