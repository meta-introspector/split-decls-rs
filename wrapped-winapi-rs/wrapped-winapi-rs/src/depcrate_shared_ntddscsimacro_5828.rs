// Generated macro for macro_5828 (macro)
macro_rules! Depcrate_shared_ntddscsimacro_5828 {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"macro_5828"}
// Dependencies: {}
STRUCT ! { struct SCSI_PASS_THROUGH32 { Length : USHORT , ScsiStatus : UCHAR , PathId : UCHAR , TargetId : UCHAR , Lun : UCHAR , CdbLength : UCHAR , SenseInfoLength : UCHAR , DataIn : UCHAR , DataTransferLength : ULONG , TimeOutValue : ULONG , DataBufferOffset : ULONG32 , SenseInfoOffset : ULONG , Cdb : [UCHAR ; 16] , } }
};
}
