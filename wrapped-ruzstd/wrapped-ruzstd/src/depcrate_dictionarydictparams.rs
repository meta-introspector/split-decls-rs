// Generated macro for DictParams (struct)
macro_rules! Depcrate_dictionaryDictParams {
() => {
// Module: crate::dictionary
// Provides: {"DictParams"}
// Dependencies: {}
# [doc = " A set of values that are used during dictionary construction."] # [doc = ""] # [doc = " Changing these values can improve the resulting dictionary size for certain datasets."] pub (super) struct DictParams { # [doc = " Segment size."] # [doc = ""] # [doc = " As found under \"4. Experiments - Varying Segment Size\" in the original paper, a"] # [doc = " segment size of 2 kiB was effective."] # [doc = ""] # [doc = " \"We explored a range of \\[`segment_size`\\] values and found the performance of LMC is insensitive"] # [doc = " to \\[`segment_size`\\]. We fix \\[`segment_size`\\] to 2kiB"] # [doc = ""] # [doc = " Reasonable range: [16, 2048+]"] pub segment_size : u32 , }
};
}
