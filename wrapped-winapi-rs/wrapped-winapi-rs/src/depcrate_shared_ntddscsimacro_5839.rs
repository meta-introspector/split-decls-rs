// Generated macro for macro_5839 (macro)
macro_rules! Depcrate_shared_ntddscsimacro_5839 {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"macro_5839"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] IFDEF ! { STRUCT ! { struct ATA_PASS_THROUGH_EX32 { Length : USHORT , AtaFlags : USHORT , PathId : UCHAR , TargetId : UCHAR , Lun : UCHAR , ReservedAsUchar : UCHAR , DataTransferLength : ULONG , TimeOutValue : ULONG , ReservedAsUlong : ULONG , DataBufferOffset : ULONG32 , PreviousTaskFile : [UCHAR ; 8] , CurrentTaskFile : [UCHAR ; 8] , } } pub type PATA_PASS_THROUGH_EX32 = * mut ATA_PASS_THROUGH_EX32 ; STRUCT ! { struct ATA_PASS_THROUGH_DIRECT32 { Length : USHORT , AtaFlags : USHORT , PathId : UCHAR , TargetId : UCHAR , Lun : UCHAR , ReservedAsUchar : UCHAR , DataTransferLength : ULONG , TimeOutValue : ULONG , ReservedAsUlong : ULONG , DataBuffer : ULONG32 , PreviousTaskFile : [UCHAR ; 8] , CurrentTaskFile : [UCHAR ; 8] , } } pub type PATA_PASS_THROUGH_DIRECT32 = * mut ATA_PASS_THROUGH_DIRECT32 ; }
};
}
