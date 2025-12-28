macro_rules! deps {
    () => {
        Span!();
        Instrument!();
    };
}

macro_rules! macro_42 {
    () => {
        deps!();
        pin_project ! { # [doc = " A [`Future`] that has been instrumented with a `tracing` [`Span`]."] # [doc = ""] # [doc = " This type is returned by the [`Instrument`] extension trait. See that"] # [doc = " trait's documentation for details."] # [doc = ""] # [doc = " [`Future`]: std::future::Future"] # [doc = " [`Span`]: crate::Span"] # [project = InstrumentedProj] # [project_ref = InstrumentedProjRef] # [derive (Debug , Clone)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Instrumented < T > { # [pin] inner : ManuallyDrop < T >, span : Span , } impl < T > PinnedDrop for Instrumented < T > { fn drop (this : Pin <& mut Self >) { let this = this . project () ; let _enter = this . span . enter () ; unsafe { ManuallyDrop :: drop (this . inner . get_unchecked_mut ()) } } } }
    };
}

macro_42!()