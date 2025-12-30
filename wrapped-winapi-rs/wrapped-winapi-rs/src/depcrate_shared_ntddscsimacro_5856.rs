// Generated macro for macro_5856 (macro)
macro_rules! Depcrate_shared_ntddscsimacro_5856 {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"macro_5856"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] IFDEF ! { STRUCT ! { struct MPIO_PASS_THROUGH_PATH32 { PassThrough : SCSI_PASS_THROUGH32 , Version : ULONG , Length : USHORT , Flags : UCHAR , PortNumber : UCHAR , MpioPathId : ULONGLONG , } } pub type PMPIO_PASS_THROUGH_PATH32 = * mut MPIO_PASS_THROUGH_PATH32 ; STRUCT ! { struct MPIO_PASS_THROUGH_PATH_DIRECT32 { PassThrough : SCSI_PASS_THROUGH_DIRECT32 , Version : ULONG , Length : USHORT , Flags : UCHAR , PortNumber : UCHAR , MpioPathId : ULONGLONG , } } pub type PMPIO_PASS_THROUGH_PATH_DIRECT32 = * mut MPIO_PASS_THROUGH_PATH_DIRECT32 ; STRUCT ! { struct MPIO_PASS_THROUGH_PATH32_EX { PassThroughOffset : ULONG , Version : ULONG , Length : USHORT , Flags : UCHAR , PortNumber : UCHAR , MpioPathId : ULONGLONG , } } pub type PMPIO_PASS_THROUGH_PATH32_EX = * mut MPIO_PASS_THROUGH_PATH32_EX ; STRUCT ! { struct MPIO_PASS_THROUGH_PATH_DIRECT32_EX { PassThroughOffset : ULONG , Version : ULONG , Length : USHORT , Flags : UCHAR , PortNumber : UCHAR , MpioPathId : ULONGLONG , } } pub type PMPIO_PASS_THROUGH_PATH_DIRECT32_EX = * mut MPIO_PASS_THROUGH_PATH_DIRECT32_EX ; }
};
}
