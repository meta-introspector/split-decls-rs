// Generated macro for macro_5830 (macro)
macro_rules! Depcrate_shared_ntddscsimacro_5830 {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"macro_5830"}
// Dependencies: {}
STRUCT ! { struct SCSI_PASS_THROUGH_EX { Version : ULONG , Length : ULONG , CdbLength : ULONG , StorAddressLength : ULONG , ScsiStatus : UCHAR , SenseInfolength : UCHAR , DataDirection : UCHAR , Reserved : UCHAR , TimeOutValue : ULONG , StorAddressOffset : ULONG , SenseInfoOffset : ULONG , DataOutTransferLength : ULONG , DataInTransferLength : ULONG , DataOutBufferOffset : ULONG_PTR , DataInBufferOffset : ULONG_PTR , Cdb : [UCHAR ; ANYSIZE_ARRAY] , } }
};
}
