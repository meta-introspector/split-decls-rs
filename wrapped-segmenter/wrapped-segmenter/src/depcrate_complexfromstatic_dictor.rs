// Generated macro for fromstatic_dictor (function)
macro_rules! Depcrate_complexfromstatic_dictor {
() => {
// Module: crate::complex
// Provides: {"fromstatic_dictor"}
// Dependencies: {}
fn fromstatic_dictor (dict_or : DictOrLstmBorrowed < 'static >) -> DictOrLstm { match dict_or { DictOrLstmBorrowed :: Dict (dict) => DictOrLstm :: Dict (DataPayload :: from_static_ref (dict)) , # [cfg (feature = "lstm")] DictOrLstmBorrowed :: Lstm (lstm) => DictOrLstm :: Lstm (DataPayload :: from_static_ref (lstm)) , } }
};
}
