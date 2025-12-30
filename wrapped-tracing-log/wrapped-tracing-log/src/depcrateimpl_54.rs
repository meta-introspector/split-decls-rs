// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'a > AsTrace for log :: Metadata < 'a > { type Trace = Metadata < 'a > ; fn as_trace (& self) -> Self :: Trace { let cs_id = identify_callsite ! (loglevel_to_cs (self . level ()) . 0) ; Metadata :: new ("log record" , self . target () , self . level () . as_trace () , None , None , None , field :: FieldSet :: new (FIELD_NAMES , cs_id) , Kind :: EVENT ,) } }
};
}
