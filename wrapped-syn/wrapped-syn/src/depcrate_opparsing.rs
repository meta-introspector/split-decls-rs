// Generated macro for parsing (module)
macro_rules! Depcrate_opparsing {
() => {
// Module: crate::op
// Provides: {"parsing"}
// Dependencies: {}
# [cfg (feature = "parsing")] pub mod parsing { use super :: * ; use synom :: Synom ; use synom :: tokens :: * ; impl BinOp { named ! (pub parse_binop -> Self , alt ! (syn ! (AndAnd) => { BinOp :: And } | syn ! (OrOr) => { BinOp :: Or } | syn ! (Shl) => { BinOp :: Shl } | syn ! (Shr) => { BinOp :: Shr } | syn ! (EqEq) => { BinOp :: Eq } | syn ! (Le) => { BinOp :: Le } | syn ! (Ne) => { BinOp :: Ne } | syn ! (Ge) => { BinOp :: Ge } | syn ! (Add) => { BinOp :: Add } | syn ! (Sub) => { BinOp :: Sub } | syn ! (Star) => { BinOp :: Mul } | syn ! (Div) => { BinOp :: Div } | syn ! (Rem) => { BinOp :: Rem } | syn ! (Caret) => { BinOp :: BitXor } | syn ! (And) => { BinOp :: BitAnd } | syn ! (Or) => { BinOp :: BitOr } | syn ! (Lt) => { BinOp :: Lt } | syn ! (Gt) => { BinOp :: Gt })) ; # [cfg (feature = "full")] named ! (pub parse_assign_op -> Self , alt ! (syn ! (AddEq) => { BinOp :: AddEq } | syn ! (SubEq) => { BinOp :: SubEq } | syn ! (MulEq) => { BinOp :: MulEq } | syn ! (DivEq) => { BinOp :: DivEq } | syn ! (RemEq) => { BinOp :: RemEq } | syn ! (CaretEq) => { BinOp :: BitXorEq } | syn ! (AndEq) => { BinOp :: BitAndEq } | syn ! (OrEq) => { BinOp :: BitOrEq } | syn ! (ShlEq) => { BinOp :: ShlEq } | syn ! (ShrEq) => { BinOp :: ShrEq })) ; } impl Synom for UnOp { named ! (parse -> Self , alt ! (syn ! (Star) => { UnOp :: Deref } | syn ! (Bang) => { UnOp :: Not } | syn ! (Sub) => { UnOp :: Neg })) ; } }
};
}
