macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: BinOp { type T = crate :: mir :: BinOp ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: mir :: BinOp ; match self { BinOp :: Add => crate :: mir :: BinOp :: Add , BinOp :: AddUnchecked => crate :: mir :: BinOp :: AddUnchecked , BinOp :: AddWithOverflow => bug ! ("AddWithOverflow should have been translated already") , BinOp :: Sub => crate :: mir :: BinOp :: Sub , BinOp :: SubUnchecked => crate :: mir :: BinOp :: SubUnchecked , BinOp :: SubWithOverflow => bug ! ("AddWithOverflow should have been translated already") , BinOp :: Mul => crate :: mir :: BinOp :: Mul , BinOp :: MulUnchecked => crate :: mir :: BinOp :: MulUnchecked , BinOp :: MulWithOverflow => bug ! ("AddWithOverflow should have been translated already") , BinOp :: Div => crate :: mir :: BinOp :: Div , BinOp :: Rem => crate :: mir :: BinOp :: Rem , BinOp :: BitXor => crate :: mir :: BinOp :: BitXor , BinOp :: BitAnd => crate :: mir :: BinOp :: BitAnd , BinOp :: BitOr => crate :: mir :: BinOp :: BitOr , BinOp :: Shl => crate :: mir :: BinOp :: Shl , BinOp :: ShlUnchecked => crate :: mir :: BinOp :: ShlUnchecked , BinOp :: Shr => crate :: mir :: BinOp :: Shr , BinOp :: ShrUnchecked => crate :: mir :: BinOp :: ShrUnchecked , BinOp :: Eq => crate :: mir :: BinOp :: Eq , BinOp :: Lt => crate :: mir :: BinOp :: Lt , BinOp :: Le => crate :: mir :: BinOp :: Le , BinOp :: Ne => crate :: mir :: BinOp :: Ne , BinOp :: Ge => crate :: mir :: BinOp :: Ge , BinOp :: Gt => crate :: mir :: BinOp :: Gt , BinOp :: Cmp => crate :: mir :: BinOp :: Cmp , BinOp :: Offset => crate :: mir :: BinOp :: Offset , } } }
    };
}

impl_149!();