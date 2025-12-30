// Generated macro for DictOrLstmBorrowed (enum)
macro_rules! Depcrate_complexDictOrLstmBorrowed {
() => {
// Module: crate::complex
// Provides: {"DictOrLstmBorrowed"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] enum DictOrLstmBorrowed < 'data > { Dict (& 'data UCharDictionaryBreakData < 'data >) , # [cfg (feature = "lstm")] Lstm (& 'data LstmData < 'data >) , }
};
}
