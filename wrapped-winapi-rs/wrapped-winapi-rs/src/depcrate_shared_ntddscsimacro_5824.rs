// Generated macro for macro_5824 (macro)
macro_rules! Depcrate_shared_ntddscsimacro_5824 {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"macro_5824"}
// Dependencies: {}
STRUCT ! { struct SCSI_PASS_THROUGH { Length : USHORT , ScsiStatus : UCHAR , PathId : UCHAR , TargetId : UCHAR , Lun : UCHAR , CdbLength : UCHAR , SenseInfoLength : UCHAR , DataIn : UCHAR , DataTransferLength : ULONG , TimeOutValue : ULONG , DataBufferOffset : ULONG_PTR , SenseInfoOffset : ULONG , Cdb : [UCHAR ; 16] , } }
};
}
