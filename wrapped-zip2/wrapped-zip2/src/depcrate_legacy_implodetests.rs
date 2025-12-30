// Generated macro for tests (module)
macro_rules! Depcrate_legacy_implodetests {
() => {
// Module: crate::legacy::implode
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: hwexplode ; const HAMLET_256 : & [u8 ; 249] = include_bytes ! ("../../tests/data/legacy/implode_hamlet_256.bin") ; const HAMLET_256_OUT : & [u8 ; 256] = include_bytes ! ("../../tests/data/legacy/implode_hamlet_256.out") ; # [test] fn test_explode_hamlet_256 () { let mut dst = Vec :: new () ; hwexplode (HAMLET_256 , 256 , false , false , false , & mut dst) . unwrap () ; assert_eq ! (dst . len () , 256) ; assert_eq ! (& dst , & HAMLET_256_OUT) ; } }
};
}
