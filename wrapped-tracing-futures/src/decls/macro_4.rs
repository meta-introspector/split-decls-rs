macro_rules! deps {
    () => {
        Instrumented!();
    };
}

macro_rules! macro_4 {
    () => {
        deps!();
        # [cfg (feature = "std-future")] pin_project ! { # [doc = " A future, stream, sink, or executor that has been instrumented with a `tracing` span."] # [project = InstrumentedProj] # [project_ref = InstrumentedProjRef] # [derive (Debug , Clone)] pub struct Instrumented < T > { # [pin] inner : ManuallyDrop < T >, span : Span , } impl < T > PinnedDrop for Instrumented < T > { fn drop (this : Pin <& mut Self >) { let this = this . project () ; let _enter = this . span . enter () ; unsafe { ManuallyDrop :: drop (this . inner . get_unchecked_mut ()) } } } }
    };
}

macro_4!()