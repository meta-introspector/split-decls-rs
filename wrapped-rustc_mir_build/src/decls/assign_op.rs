macro_rules! assign_op {
    () => {
        fn assign_op (op : hir :: AssignOpKind) -> AssignOp { match op { hir :: AssignOpKind :: AddAssign => AssignOp :: AddAssign , hir :: AssignOpKind :: SubAssign => AssignOp :: SubAssign , hir :: AssignOpKind :: MulAssign => AssignOp :: MulAssign , hir :: AssignOpKind :: DivAssign => AssignOp :: DivAssign , hir :: AssignOpKind :: RemAssign => AssignOp :: RemAssign , hir :: AssignOpKind :: BitXorAssign => AssignOp :: BitXorAssign , hir :: AssignOpKind :: BitAndAssign => AssignOp :: BitAndAssign , hir :: AssignOpKind :: BitOrAssign => AssignOp :: BitOrAssign , hir :: AssignOpKind :: ShlAssign => AssignOp :: ShlAssign , hir :: AssignOpKind :: ShrAssign => AssignOp :: ShrAssign , } }
    };
}

assign_op!();