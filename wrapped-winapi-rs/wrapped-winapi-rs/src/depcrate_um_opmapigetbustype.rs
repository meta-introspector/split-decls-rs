// Generated macro for GetBusType (function)
macro_rules! Depcrate_um_opmapiGetBusType {
() => {
// Module: crate::um::opmapi
// Provides: {"GetBusType"}
// Dependencies: {}
# [inline] pub fn GetBusType (ulBusTypeAndImplementation : ULONG) -> ULONG { ulBusTypeAndImplementation & OPM_BUS_TYPE_MASK }
};
}
