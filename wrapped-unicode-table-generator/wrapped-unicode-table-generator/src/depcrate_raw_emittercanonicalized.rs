// Generated macro for Canonicalized (struct)
macro_rules! Depcrate_raw_emitterCanonicalized {
() => {
// Module: crate::raw_emitter
// Provides: {"Canonicalized"}
// Dependencies: {}
struct Canonicalized { canonical_words : Vec < u64 > , canonicalized_words : Vec < (u8 , u8) > , # [doc = " Maps an input unique word to the associated index (u8) which is into"] # [doc = " canonical_words or canonicalized_words (in order)."] unique_mapping : HashMap < u64 , u8 > , }
};
}
