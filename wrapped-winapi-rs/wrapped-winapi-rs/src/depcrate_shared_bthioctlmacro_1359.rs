// Generated macro for macro_1359 (macro)
macro_rules! Depcrate_shared_bthioctlmacro_1359 {
() => {
// Module: crate::shared::bthioctl
// Provides: {"macro_1359"}
// Dependencies: {}
STRUCT ! { # [repr (packed)] struct BTH_SDP_SERVICE_ATTRIBUTE_SEARCH_REQUEST { hConnection : HANDLE_SDP_TYPE , searchFlags : ULONG , uuids : [SdpQueryUuid ; MAX_UUIDS_IN_QUERY] , range : [SdpAttributeRange ; 1] , } }
};
}
