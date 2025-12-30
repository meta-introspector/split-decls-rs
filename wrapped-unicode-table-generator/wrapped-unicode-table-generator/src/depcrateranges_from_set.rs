// Generated macro for ranges_from_set (function)
macro_rules! Depcrateranges_from_set {
() => {
// Module: crate
// Provides: {"ranges_from_set"}
// Dependencies: {}
# [doc = " Group the elements of `set` into contigous ranges"] fn ranges_from_set (set : & [u32]) -> Vec < Range < u32 > > { set . chunk_by (| a , b | a + 1 == * b) . map (| chunk | { let start = * chunk . first () . unwrap () ; let end = * chunk . last () . unwrap () ; start .. (end + 1) }) . collect () }
};
}
