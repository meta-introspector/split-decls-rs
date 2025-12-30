// Generated macro for impl_72 (impl)
macro_rules! Depcrateimpl_72 {
() => {
// Module: crate
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'a > AsTrace for log :: Record < 'a > { type Trace = Metadata < 'a > ; fn as_trace (& self) -> Self :: Trace { let cs_id = identify_callsite ! (loglevel_to_cs (self . level ()) . 0) ; Metadata :: new ("log record" , self . target () , self . level () . as_trace () , self . file () , self . line () , self . module_path () , field :: FieldSet :: new (FIELD_NAMES , cs_id) , Kind :: EVENT ,) } }
};
}
