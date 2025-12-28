macro_rules! deps {
    () => {
        QueryJobId!();
        QueryJob!();
        QueryLatch!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < I > QueryJob < I > { # [doc = " Creates a new query job."] # [inline] pub fn new (id : QueryJobId , span : Span , parent : Option < QueryJobId >) -> Self { QueryJob { id , span , parent , latch : None } } pub (super) fn latch (& mut self) -> QueryLatch < I > { if self . latch . is_none () { self . latch = Some (QueryLatch :: new ()) ; } self . latch . as_ref () . unwrap () . clone () } # [doc = " Signals to waiters that the query is complete."] # [doc = ""] # [doc = " This does nothing for single threaded rustc,"] # [doc = " as there are no concurrent jobs which could be waiting on us"] # [inline] pub fn signal_complete (self) { if let Some (latch) = self . latch { latch . set () ; } } }
    };
}

impl_172!();