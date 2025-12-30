// Generated macro for GetBusImplementation (function)
macro_rules! Depcrate_um_opmapiGetBusImplementation {
() => {
// Module: crate::um::opmapi
// Provides: {"GetBusImplementation"}
// Dependencies: {}
# [inline] pub fn GetBusImplementation (ulBusTypeAndImplementation : ULONG) -> ULONG { (ulBusTypeAndImplementation & OPM_BUS_IMPLEMENTATION_MODIFIER_MASK) >> 16 }
};
}
