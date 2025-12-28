macro_rules! deps {
    () => {
        BlockFrame!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl BlockFrame { fn is_tail_expr (& self) -> bool { match * self { BlockFrame :: TailExpr { .. } => true , BlockFrame :: Statement { .. } | BlockFrame :: SubExpr => false , } } fn is_statement (& self) -> bool { match * self { BlockFrame :: Statement { .. } => true , BlockFrame :: TailExpr { .. } | BlockFrame :: SubExpr => false , } } }
    };
}

impl_3!()