// Generated macro for IsNonStandardBusImplementation (function)
macro_rules! Depcrate_um_opmapiIsNonStandardBusImplementation {
() => {
// Module: crate::um::opmapi
// Provides: {"IsNonStandardBusImplementation"}
// Dependencies: {}
# [inline] pub fn IsNonStandardBusImplementation (ulBusTypeAndImplementation : ULONG) -> ULONG { ulBusTypeAndImplementation & OPM_BUS_IMPLEMENTATION_MODIFIER_NON_STANDARD }
};
}
