// Generated macro for macro_5834 (macro)
macro_rules! Depcrate_shared_ntddscsimacro_5834 {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"macro_5834"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] IFDEF ! { STRUCT ! { struct SCSI_PASS_THROUGH32_EX { Version : ULONG , Length : ULONG , CdbLength : ULONG , StorAddressLength : ULONG , ScsiStatus : UCHAR , SenseInfolength : UCHAR , DataDirection : UCHAR , Reserved : UCHAR , TimeOutValue : ULONG , StorAddressOffset : ULONG , SenseInfoOffset : ULONG , DataOutTransferLength : ULONG , DataInTransferLength : ULONG , DataOutBufferOffset : ULONG32 , DataInBufferOffset : ULONG32 , Cdb : [UCHAR ; ANYSIZE_ARRAY] , } } pub type PSCSI_PASS_THROUGH32_EX = * mut SCSI_PASS_THROUGH32_EX ; STRUCT ! { struct SCSI_PASS_THROUGH_DIRECT32_EX { Version : ULONG , Length : ULONG , CdbLength : ULONG , StorAddressLength : ULONG , ScsiStatus : UCHAR , SenseInfolength : UCHAR , DataDirection : UCHAR , Reserved : UCHAR , TimeOutValue : ULONG , StorAddressOffset : ULONG , SenseInfoOffset : ULONG , DataOutTransferLength : ULONG , DataInTransferLength : ULONG , DataOutBuffer : ULONG32 , DataInBuffer : ULONG32 , Cdb : [UCHAR ; ANYSIZE_ARRAY] , } } pub type PSCSI_PASS_THROUGH_DIRECT32_EX = * mut SCSI_PASS_THROUGH_DIRECT32_EX ; }
};
}
