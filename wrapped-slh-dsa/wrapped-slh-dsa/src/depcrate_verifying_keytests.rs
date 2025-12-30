// Generated macro for tests (module)
macro_rules! Depcrate_verifying_keytests {
() => {
// Module: crate::verifying_key
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: * ; use hybrid_array :: Array ; use signature :: * ; # [test] fn test_vk_serialize_deserialize () { let mut rng = rand :: rng () ; let sk = SigningKey :: < Shake128f > :: new (& mut rng) ; let vk = sk . verifying_key () ; let vk_bytes : Array < u8 , _ > = (& vk) . into () ; let vk2 = VerifyingKey :: < Shake128f > :: try_from (vk_bytes . as_slice ()) . unwrap () ; assert_eq ! (vk , vk2) ; } }
};
}
