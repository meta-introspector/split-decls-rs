macro_rules! RPITVisitor {
    () => {
        struct RPITVisitor < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , synthetics : Vec < LocalDefId > , data : DefPathData , disambiguator : & 'a mut DisambiguatorState , }
    };
}

RPITVisitor!();