macro_rules! bin_op {
    () => {
        fn bin_op (op : hir :: BinOpKind) -> BinOp { match op { hir :: BinOpKind :: Add => BinOp :: Add , hir :: BinOpKind :: Sub => BinOp :: Sub , hir :: BinOpKind :: Mul => BinOp :: Mul , hir :: BinOpKind :: Div => BinOp :: Div , hir :: BinOpKind :: Rem => BinOp :: Rem , hir :: BinOpKind :: BitXor => BinOp :: BitXor , hir :: BinOpKind :: BitAnd => BinOp :: BitAnd , hir :: BinOpKind :: BitOr => BinOp :: BitOr , hir :: BinOpKind :: Shl => BinOp :: Shl , hir :: BinOpKind :: Shr => BinOp :: Shr , hir :: BinOpKind :: Eq => BinOp :: Eq , hir :: BinOpKind :: Lt => BinOp :: Lt , hir :: BinOpKind :: Le => BinOp :: Le , hir :: BinOpKind :: Ne => BinOp :: Ne , hir :: BinOpKind :: Ge => BinOp :: Ge , hir :: BinOpKind :: Gt => BinOp :: Gt , _ => bug ! ("no equivalent for ast binop {:?}" , op) , } }
    };
}

bin_op!()