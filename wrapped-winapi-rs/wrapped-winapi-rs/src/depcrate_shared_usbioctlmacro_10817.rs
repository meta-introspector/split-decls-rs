// Generated macro for macro_10817 (macro)
macro_rules! Depcrate_shared_usbioctlmacro_10817 {
() => {
// Module: crate::shared::usbioctl
// Provides: {"macro_10817"}
// Dependencies: {}
STRUCT ! { # [repr (packed)] struct HCD_ISO_STAT_COUNTERS { LateUrbs : USHORT , DoubleBufferedPackets : USHORT , TransfersCF_5ms : USHORT , TransfersCF_2ms : USHORT , TransfersCF_1ms : USHORT , MaxInterruptLatency : USHORT , BadStartFrame : USHORT , StaleUrbs : USHORT , IsoPacketNotAccesed : USHORT , IsoPacketHWError : USHORT , SmallestUrbPacketCount : USHORT , LargestUrbPacketCount : USHORT , IsoCRC_Error : USHORT , IsoOVERRUN_Error : USHORT , IsoINTERNAL_Error : USHORT , IsoUNKNOWN_Error : USHORT , IsoBytesTransferred : ULONG , LateMissedCount : USHORT , HWIsoMissedCount : USHORT , Reserved7 : [ULONG ; 8] , } }
};
}
