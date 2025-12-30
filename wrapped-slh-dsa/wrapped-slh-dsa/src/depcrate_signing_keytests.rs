// Generated macro for tests (module)
macro_rules! Depcrate_signing_keytests {
() => {
// Module: crate::signing_key
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { ParameterSet , SigningKey , util :: macros :: test_parameter_sets } ; fn test_serialize_deserialize < P : ParameterSet > () { let mut rng : rand :: prelude :: ThreadRng = rand :: rng () ; let sk = SigningKey :: < P > :: new (& mut rng) ; let bytes = sk . to_bytes () ; let sk2 = SigningKey :: < P > :: try_from (bytes . as_slice ()) . unwrap () ; assert_eq ! (sk , sk2) ; } test_parameter_sets ! (test_serialize_deserialize) ; # [cfg (feature = "alloc")] fn test_serialize_deserialize_vec < P : ParameterSet > () { let mut rng : rand :: prelude :: ThreadRng = rand :: rng () ; let sk = SigningKey :: < P > :: new (& mut rng) ; let vec = sk . to_vec () ; let sk2 = SigningKey :: < P > :: try_from (vec . as_slice ()) . unwrap () ; assert_eq ! (sk , sk2) ; } # [cfg (feature = "alloc")] test_parameter_sets ! (test_serialize_deserialize_vec) ; # [test] fn test_deserialize_fail_on_incorrect_length () { let mut rng : rand :: prelude :: ThreadRng = rand :: rng () ; let sk = SigningKey :: < Shake128f > :: new (& mut rng) ; let bytes = sk . to_bytes () ; let incorrect_bytes = & bytes [.. bytes . len () - 1] ; assert ! (SigningKey ::< Shake128f >:: try_from (incorrect_bytes) . is_err ()) ; } }
};
}
