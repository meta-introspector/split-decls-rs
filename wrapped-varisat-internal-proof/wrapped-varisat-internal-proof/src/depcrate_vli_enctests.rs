// Generated macro for tests (module)
macro_rules! Depcrate_vli_enctests {
() => {
// Module: crate::vli_enc
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use proptest :: prelude :: * ; proptest ! { # [test] fn roundtrip (numbers in prop :: collection :: vec (prop :: num :: u64 :: ANY , 0 .. 10_000)) { let mut buf = vec ! [] ; for & num in numbers . iter () { write_u64 (& mut buf , num) ?; } let mut read = std :: io :: BufReader :: with_capacity (128 , & buf [..]) ; let mut out = vec ! [] ; while let Ok (num) = read_u64 (& mut read) { out . push (num) } prop_assert_eq ! (numbers , out) ; } } }
};
}
