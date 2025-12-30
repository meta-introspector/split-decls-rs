// Generated macro for Value (struct)
macro_rules! Depcrate_inputValue {
() => {
// Module: crate::input
// Provides: {"Value"}
// Dependencies: {}
# [derive (Debug)] pub struct Value < C > where C : Configuration , { # [doc = " Fields of this input struct."] # [doc = ""] # [doc = " They can change across revisions, but they do not change within"] # [doc = " a particular revision."] fields : C :: Fields , # [doc = " Revisions of the fields."] revisions : C :: Revisions , # [doc = " Durabilities of the fields."] durabilities : C :: Durabilities , # [doc = " Memos"] memos : MemoTable , }
};
}
