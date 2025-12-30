// Generated macro for BinExpr (struct)
macro_rules! Depcrate_astBinExpr {
() => {
// Module: crate::ast
// Provides: {"BinExpr"}
// Dependencies: {}
# [doc = " A binary expression (e.g. `$x + 1`)."] # [derive (Debug , PartialEq)] pub (crate) struct BinExpr < O , L , R = L > { left : Box < L > , right : Box < R > , op : O , }
};
}
