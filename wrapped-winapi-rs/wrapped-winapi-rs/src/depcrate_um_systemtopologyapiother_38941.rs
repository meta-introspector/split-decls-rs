// Generated macro for other_38941 (other)
macro_rules! Depcrate_um_systemtopologyapiother_38941 {
() => {
// Module: crate::um::systemtopologyapi
// Provides: {"other_38941"}
// Dependencies: {}
extern "system" { pub fn GetNumaHighestNodeNumber (HighestNodeNumber : PULONG ,) -> BOOL ; pub fn GetNumaNodeProcessorMaskEx (Node : USHORT , ProcessorMask : PGROUP_AFFINITY ,) -> BOOL ; pub fn GetNumaProximityNodeEx (ProximityId : ULONG , NodeNumber : PUSHORT ,) -> BOOL ; }
};
}
