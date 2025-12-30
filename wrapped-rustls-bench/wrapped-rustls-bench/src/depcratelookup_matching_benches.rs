// Generated macro for lookup_matching_benches (function)
macro_rules! Depcratelookup_matching_benches {
() => {
// Module: crate
// Provides: {"lookup_matching_benches"}
// Dependencies: {}
fn lookup_matching_benches (ciphersuite_name : & str , key_type : Option < RequestedKeyType > , provider : & Provider ,) -> Vec < BenchmarkParam > { let r : Vec < BenchmarkParam > = ALL_BENCHMARKS . iter () . filter (| params | { format ! ("{:?}" , params . ciphersuite) . to_lowercase () == ciphersuite_name . to_lowercase () && (key_type . is_none () || Some (params . key_type) == key_type . map (KeyType :: from)) && provider . supports_benchmark (params) }) . cloned () . collect () ; if r . is_empty () { panic ! ("unknown suite {ciphersuite_name:?}") ; } r }
};
}
