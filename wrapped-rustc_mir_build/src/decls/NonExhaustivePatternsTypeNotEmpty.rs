macro_rules! NonExhaustivePatternsTypeNotEmpty {
    () => {
        pub (crate) struct NonExhaustivePatternsTypeNotEmpty < 'p , 'tcx , 'm > { pub (crate) cx : & 'm RustcPatCtxt < 'p , 'tcx > , pub (crate) scrut_span : Span , pub (crate) braces_span : Option < Span > , pub (crate) ty : Ty < 'tcx > , }
    };
}

NonExhaustivePatternsTypeNotEmpty!();