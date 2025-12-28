macro_rules! check_binop {
    () => {
        # [doc = " We do not allow all binary operations in abstract consts, so filter disallowed ones."] fn check_binop (op : mir :: BinOp) -> bool { use mir :: BinOp :: * ; match op { Add | AddUnchecked | AddWithOverflow | Sub | SubUnchecked | SubWithOverflow | Mul | MulUnchecked | MulWithOverflow | Div | Rem | BitXor | BitAnd | BitOr | Shl | ShlUnchecked | Shr | ShrUnchecked | Eq | Lt | Le | Ne | Ge | Gt | Cmp => true , Offset => false , } }
    };
}

check_binop!();