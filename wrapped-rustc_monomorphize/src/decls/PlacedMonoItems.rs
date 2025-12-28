macro_rules! PlacedMonoItems {
    () => {
        struct PlacedMonoItems < 'tcx > { # [doc = " The codegen units, sorted by name to make things deterministic."] codegen_units : Vec < CodegenUnit < 'tcx > > , internalization_candidates : UnordSet < MonoItem < 'tcx > > , }
    };
}

PlacedMonoItems!();