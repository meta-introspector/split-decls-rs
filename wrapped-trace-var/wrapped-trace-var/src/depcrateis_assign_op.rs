// Generated macro for is_assign_op (function)
macro_rules! Depcrateis_assign_op {
() => {
// Module: crate
// Provides: {"is_assign_op"}
// Dependencies: {}
fn is_assign_op (op : BinOp) -> bool { match op { BinOp :: AddAssign (_) | BinOp :: SubAssign (_) | BinOp :: MulAssign (_) | BinOp :: DivAssign (_) | BinOp :: RemAssign (_) | BinOp :: BitXorAssign (_) | BinOp :: BitAndAssign (_) | BinOp :: BitOrAssign (_) | BinOp :: ShlAssign (_) | BinOp :: ShrAssign (_) => true , _ => false , } }
};
}
