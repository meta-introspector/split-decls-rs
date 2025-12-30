// Generated macro for macro_5832 (macro)
macro_rules! Depcrate_shared_ntddscsimacro_5832 {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"macro_5832"}
// Dependencies: {}
STRUCT ! { struct SCSI_PASS_THROUGH_DIRECT_EX { Version : ULONG , Length : ULONG , CdbLength : ULONG , StorAddressLength : ULONG , ScsiStatus : UCHAR , SenseInfolength : UCHAR , DataDirection : UCHAR , Reserved : UCHAR , TimeOutValue : ULONG , StorAddressOffset : ULONG , SenseInfoOffset : ULONG , DataOutTransferLength : ULONG , DataInTransferLength : ULONG , DataOutBuffer : * mut VOID , DataInBuffer : * mut VOID , Cdb : [UCHAR ; ANYSIZE_ARRAY] , } }
};
}
