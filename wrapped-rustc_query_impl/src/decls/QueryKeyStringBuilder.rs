macro_rules! deps {
    () => {
        QueryKeyStringCache!();
    };
}

macro_rules! QueryKeyStringBuilder {
    () => {
        deps!();
        struct QueryKeyStringBuilder < 'a , 'tcx > { profiler : & 'a SelfProfiler , tcx : TyCtxt < 'tcx > , string_cache : & 'a mut QueryKeyStringCache , }
    };
}

QueryKeyStringBuilder!();