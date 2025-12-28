macro_rules! deps {
    () => {
        QueryCtxt!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < 'tcx > HasDepContext for QueryCtxt < 'tcx > { type Deps = rustc_middle :: dep_graph :: DepsType ; type DepContext = TyCtxt < 'tcx > ; # [inline] fn dep_context (& self) -> & Self :: DepContext { & self . tcx } }
    };
}

impl_3!();