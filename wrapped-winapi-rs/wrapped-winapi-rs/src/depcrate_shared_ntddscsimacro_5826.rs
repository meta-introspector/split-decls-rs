// Generated macro for macro_5826 (macro)
macro_rules! Depcrate_shared_ntddscsimacro_5826 {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"macro_5826"}
// Dependencies: {}
STRUCT ! { struct SCSI_PASS_THROUGH_DIRECT { Length : USHORT , ScsiStatus : UCHAR , PathId : UCHAR , TargetId : UCHAR , Lun : UCHAR , CdbLength : UCHAR , SenseInfoLength : UCHAR , DataIn : UCHAR , DataTransferLength : ULONG , TimeOutValue : ULONG , DataBuffer : PVOID , SenseInfoOffset : ULONG , Cdb : [UCHAR ; 16] , } }
};
}
