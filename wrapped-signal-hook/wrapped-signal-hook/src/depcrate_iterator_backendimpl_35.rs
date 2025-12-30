// Generated macro for impl_35 (impl)
macro_rules! Depcrate_iterator_backendimpl_35 {
() => {
// Module: crate::iterator::backend
// Provides: {"impl_35"}
// Dependencies: {}
impl < E : Exfiltrator > Debug for PendingSignals < E > { fn fmt (& self , fmt : & mut Formatter) -> FmtResult { fmt . debug_struct ("PendingSignals") . field ("exfiltrator" , & self . exfiltrator) . field ("slots" , & & self . slots [..]) . finish () } }
};
}
