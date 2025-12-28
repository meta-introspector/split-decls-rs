macro_rules! deps {
    () => {
        Scope!();
        Category!();
        RvalueFunc!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl Category { # [doc = " Determines the category for a given expression. Note that scope"] # [doc = " and paren expressions have no category."] pub (crate) fn of (ek : & ExprKind < '_ >) -> Option < Category > { match * ek { ExprKind :: Scope { .. } => None , ExprKind :: Field { .. } | ExprKind :: Deref { .. } | ExprKind :: Index { .. } | ExprKind :: UpvarRef { .. } | ExprKind :: VarRef { .. } | ExprKind :: PlaceTypeAscription { .. } | ExprKind :: ValueTypeAscription { .. } | ExprKind :: PlaceUnwrapUnsafeBinder { .. } | ExprKind :: ValueUnwrapUnsafeBinder { .. } => Some (Category :: Place) , ExprKind :: LogicalOp { .. } | ExprKind :: Match { .. } | ExprKind :: If { .. } | ExprKind :: Let { .. } | ExprKind :: NeverToAny { .. } | ExprKind :: Use { .. } | ExprKind :: Adt { .. } | ExprKind :: Borrow { .. } | ExprKind :: RawBorrow { .. } | ExprKind :: Yield { .. } | ExprKind :: Call { .. } | ExprKind :: ByUse { .. } | ExprKind :: InlineAsm { .. } => Some (Category :: Rvalue (RvalueFunc :: Into)) , ExprKind :: Array { .. } | ExprKind :: Tuple { .. } | ExprKind :: Closure { .. } | ExprKind :: Unary { .. } | ExprKind :: Binary { .. } | ExprKind :: Box { .. } | ExprKind :: Cast { .. } | ExprKind :: PointerCoercion { .. } | ExprKind :: Repeat { .. } | ExprKind :: Assign { .. } | ExprKind :: AssignOp { .. } | ExprKind :: ThreadLocalRef (_) | ExprKind :: OffsetOf { .. } | ExprKind :: WrapUnsafeBinder { .. } => Some (Category :: Rvalue (RvalueFunc :: AsRvalue)) , ExprKind :: ConstBlock { .. } | ExprKind :: Literal { .. } | ExprKind :: NonHirLiteral { .. } | ExprKind :: ZstLiteral { .. } | ExprKind :: ConstParam { .. } | ExprKind :: StaticRef { .. } | ExprKind :: NamedConst { .. } => Some (Category :: Constant) , ExprKind :: Loop { .. } | ExprKind :: LoopMatch { .. } | ExprKind :: Block { .. } | ExprKind :: Break { .. } | ExprKind :: Continue { .. } | ExprKind :: ConstContinue { .. } | ExprKind :: Return { .. } | ExprKind :: Become { .. } => { Some (Category :: Rvalue (RvalueFunc :: Into)) } } } }
    };
}

impl_79!()