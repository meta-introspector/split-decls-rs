// Generated macro for macro_4777 (macro)
macro_rules! Depcrate_shared_ipmibmacro_4777 {
() => {
// Module: crate::shared::ipmib
// Provides: {"macro_4777"}
// Dependencies: {}
STRUCT ! { struct MIB_IPMCAST_MFE { dwGroup : DWORD , dwSource : DWORD , dwSrcMask : DWORD , dwUpStrmNgbr : DWORD , dwInIfIndex : DWORD , dwInIfProtocol : DWORD , dwRouteProtocol : DWORD , dwRouteNetwork : DWORD , dwRouteMask : DWORD , ulUpTime : ULONG , ulExpiryTime : ULONG , ulTimeOut : ULONG , ulNumOutIf : ULONG , fFlags : DWORD , dwReserved : DWORD , rgmioOutInfo : [MIB_IPMCAST_OIF ; ANY_SIZE] , } }
};
}
