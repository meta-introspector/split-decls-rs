// Generated macro for impl_179 (impl)
macro_rules! Depcrate_lintimpl_179 {
() => {
// Module: crate::lint
// Provides: {"impl_179"}
// Dependencies: {}
impl < 'a , 'tcx > Lint < 'a , 'tcx > { # [track_caller] fn fail (& self , location : Location , msg : impl AsRef < str >) { let span = self . body . source_info (location) . span ; self . tcx . sess . dcx () . span_delayed_bug (span , format ! ("broken MIR in {:?} ({}) at {:?}:\n{}" , self . body . source . instance , self . when , location , msg . as_ref ()) ,) ; } }
};
}
