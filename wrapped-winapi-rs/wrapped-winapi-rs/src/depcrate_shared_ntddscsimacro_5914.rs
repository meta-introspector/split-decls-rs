// Generated macro for macro_5914 (macro)
macro_rules! Depcrate_shared_ntddscsimacro_5914 {
() => {
// Module: crate::shared::ntddscsi
// Provides: {"macro_5914"}
// Dependencies: {}
STRUCT ! { struct DSM_NOTIFICATION_REQUEST_BLOCK { Size : ULONG , Version : ULONG , NotifyFlags : ULONG , DataSetProfile : ULONG , Reserved : [ULONG ; 3] , DataSetRangesCount : ULONG , DataSetRanges : [MP_DEVICE_DATA_SET_RANGE ; ANYSIZE_ARRAY] , } }
};
}
