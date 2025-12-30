// Generated macro for LOCKED_CALLSITES (static)
macro_rules! Depcrate_callsiteLOCKED_CALLSITES {
() => {
// Module: crate::callsite
// Provides: {"LOCKED_CALLSITES"}
// Dependencies: {}
static LOCKED_CALLSITES : Lazy < Mutex < Vec < & 'static dyn Callsite > > > = Lazy :: new (Default :: default) ;
};
}
