// Generated macro for scan_left (function)
macro_rules! Depcrate_fixupscan_left {
() => {
// Module: crate::fixup
// Provides: {"scan_left"}
// Dependencies: {}
# [cfg (feature = "full")] fn scan_left (expr : & Expr , fixup : FixupContext) -> bool { match expr { Expr :: Assign (_) => fixup . previous_operator <= Precedence :: Assign , Expr :: Binary (e) => match Precedence :: of_binop (& e . op) { Precedence :: Assign => fixup . previous_operator <= Precedence :: Assign , binop_prec => fixup . previous_operator < binop_prec , } , Expr :: Cast (_) => fixup . previous_operator < Precedence :: Cast , Expr :: Range (e) => e . start . is_none () || fixup . previous_operator < Precedence :: Assign , _ => true , } }
};
}
