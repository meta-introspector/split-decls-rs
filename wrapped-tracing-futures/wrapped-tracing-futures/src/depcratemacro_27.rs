// Generated macro for macro_27 (macro)
macro_rules! Depcratemacro_27 {
() => {
// Module: crate
// Provides: {"macro_27"}
// Dependencies: {}
# [cfg (feature = "std-future")] pin_project ! { # [doc = " A future, stream, sink, or executor that has been instrumented with a `tracing` span."] # [project = InstrumentedProj] # [project_ref = InstrumentedProjRef] # [derive (Debug , Clone)] pub struct Instrumented < T > { # [pin] inner : ManuallyDrop < T >, span : Span , } impl < T > PinnedDrop for Instrumented < T > { fn drop (this : Pin <& mut Self >) { let this = this . project () ; let _enter = this . span . enter () ; unsafe { ManuallyDrop :: drop (this . inner . get_unchecked_mut ()) } } } }
};
}
