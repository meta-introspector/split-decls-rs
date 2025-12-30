// Generated macro for BinExpr (struct)
macro_rules! Depcrate_expressionBinExpr {
() => {
// Module: crate::expression
// Provides: {"BinExpr"}
// Dependencies: {}
# [doc = " A binary expression (e.g. `$x + 1`)."] # [derive (Debug)] # [decl (struct , name = "BinExpr" , vis = "pub" , hash = "97fdb191")] pub struct BinExpr < O , L , R = L > { left : Box < L > , right : Box < R > , op : O , }
};
}
