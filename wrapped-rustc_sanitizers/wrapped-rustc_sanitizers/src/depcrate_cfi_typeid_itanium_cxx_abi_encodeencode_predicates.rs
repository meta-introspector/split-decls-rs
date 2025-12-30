// Generated macro for encode_predicates (function)
macro_rules! Depcrate_cfi_typeid_itanium_cxx_abi_encodeencode_predicates {
() => {
// Module: crate::cfi::typeid::itanium_cxx_abi::encode
// Provides: {"encode_predicates"}
// Dependencies: {}
# [doc = " Encodes predicates using the Itanium C++ ABI with vendor extended type qualifiers and types for"] # [doc = " Rust types that are not used at the FFI boundary."] fn encode_predicates < 'tcx > (tcx : TyCtxt < 'tcx > , predicates : & List < ty :: PolyExistentialPredicate < 'tcx > > , dict : & mut FxHashMap < DictKey < 'tcx > , usize > , options : EncodeTyOptions ,) -> String { let mut s = String :: new () ; let predicates : Vec < ty :: PolyExistentialPredicate < 'tcx > > = predicates . iter () . collect () ; for predicate in predicates { s . push_str (& encode_predicate (tcx , predicate , dict , options)) ; } s }
};
}
