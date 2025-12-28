macro_rules! deps {
    () => {
        DictKey!();
        EncodeTyOptions!();
    };
}

macro_rules! encode_predicates {
    () => {
        deps!();
        # [doc = " Encodes predicates using the Itanium C++ ABI with vendor extended type qualifiers and types for"] # [doc = " Rust types that are not used at the FFI boundary."] fn encode_predicates < 'tcx > (tcx : TyCtxt < 'tcx > , predicates : & List < ty :: PolyExistentialPredicate < 'tcx > > , dict : & mut FxHashMap < DictKey < 'tcx > , usize > , options : EncodeTyOptions ,) -> String { let mut s = String :: new () ; let predicates : Vec < ty :: PolyExistentialPredicate < 'tcx > > = predicates . iter () . collect () ; for predicate in predicates { s . push_str (& encode_predicate (tcx , predicate , dict , options)) ; } s }
    };
}

encode_predicates!()