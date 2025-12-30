// Generated macro for borrow_dictor (function)
macro_rules! Depcrate_complexborrow_dictor {
() => {
// Module: crate::complex
// Provides: {"borrow_dictor"}
// Dependencies: {}
fn borrow_dictor (dict_or : & DictOrLstm) -> DictOrLstmBorrowed < '_ > { match dict_or { DictOrLstm :: Dict (dict) => DictOrLstmBorrowed :: Dict (dict . get ()) , # [cfg (feature = "lstm")] DictOrLstm :: Lstm (lstm) => DictOrLstmBorrowed :: Lstm (lstm . get ()) , } }
};
}
