// Generated macro for check_unop (function)
macro_rules! Depcrate_constscheck_unop {
() => {
// Module: crate::consts
// Provides: {"check_unop"}
// Dependencies: {}
# [doc = " While we currently allow all unary operations, we still want to explicitly guard against"] # [doc = " future changes here."] fn check_unop (op : mir :: UnOp) -> bool { use mir :: UnOp :: * ; match op { Not | Neg | PtrMetadata => true , } }
};
}
