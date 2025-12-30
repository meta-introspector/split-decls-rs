// Generated macro for Context (struct)
macro_rules! Depcrate_contextContext {
() => {
// Module: crate::context
// Provides: {"Context"}
// Dependencies: {}
# [doc = " Central checker data structure."] # [doc = ""] # [doc = " This struct contains all data kept by the checker. Most functions operating on multiple fields"] # [doc = " of the context use partial references provided by the `partial_ref` crate. This documents the"] # [doc = " data dependencies and makes the borrow checker happy without the overhead of passing individual"] # [doc = " references."] # [derive (PartialRefTarget , Default)] pub struct Context < 'a > { # [part (CheckerStateP)] pub checker_state : CheckerState , # [part (ClauseHasherP)] pub clause_hasher : ClauseHasher , # [part (ClausesP)] pub clauses : Clauses , # [part (ProcessingP <'a >)] pub processing : Processing < 'a > , # [part (RupCheckP)] pub rup_check : RupCheck , # [part (TmpDataP)] pub tmp_data : TmpData , # [part (VariablesP)] pub variables : Variables , }
};
}
