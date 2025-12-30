// Generated macro for CaseClause (struct)
macro_rules! Depcrate_astCaseClause {
() => {
// Module: crate::ast
// Provides: {"CaseClause"}
// Dependencies: {}
# [doc = " Clause in a [case command](CaseCmd)."] # [derive (Debug , PartialEq)] pub (crate) struct CaseClause { pattern : Vec < Word > , cmd : Option < Term > , sep : Option < ClauseSep > , }
};
}
