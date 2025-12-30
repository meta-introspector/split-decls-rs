// Generated macro for DictOrLstm (enum)
macro_rules! Depcrate_complexDictOrLstm {
() => {
// Module: crate::complex
// Provides: {"DictOrLstm"}
// Dependencies: {}
# [derive (Debug , Clone)] # [expect (clippy :: large_enum_variant)] enum DictOrLstm { Dict (DataPayload < UCharDictionaryBreakDataV1 >) , # [cfg (feature = "lstm")] Lstm (DataPayload < SegmenterLstmAutoV1 >) , }
};
}
