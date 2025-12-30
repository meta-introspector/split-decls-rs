// Generated macro for macro_5829 (macro)
macro_rules! Depcrate_shared_ntddscsimacro_5829 {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"macro_5829"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] IFDEF ! { pub type PSCSI_PASS_THROUGH32 = * mut SCSI_PASS_THROUGH32 ; STRUCT ! { struct SCSI_PASS_THROUGH_DIRECT32 { Length : USHORT , ScsiStatus : UCHAR , PathId : UCHAR , TargetId : UCHAR , Lun : UCHAR , CdbLength : UCHAR , SenseInfoLength : UCHAR , DataIn : UCHAR , DataTransferLength : ULONG , TimeOutValue : ULONG , DataBuffer : ULONG32 , SenseInfoOffset : ULONG , Cdb : [UCHAR ; 16] , } } pub type PSCSI_PASS_THROUGH_DIRECT32 = * mut SCSI_PASS_THROUGH_DIRECT32 ; }
};
}
