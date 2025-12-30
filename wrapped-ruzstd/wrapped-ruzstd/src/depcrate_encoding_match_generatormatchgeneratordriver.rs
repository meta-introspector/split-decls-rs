// Generated macro for MatchGeneratorDriver (struct)
macro_rules! Depcrate_encoding_match_generatorMatchGeneratorDriver {
() => {
// Module: crate::encoding::match_generator
// Provides: {"MatchGeneratorDriver"}
// Dependencies: {}
# [doc = " This is the default implementation of the `Matcher` trait. It allocates and reuses the buffers when possible."] pub struct MatchGeneratorDriver { vec_pool : Vec < Vec < u8 > > , suffix_pool : Vec < SuffixStore > , match_generator : MatchGenerator , slice_size : usize , }
};
}
