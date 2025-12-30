// Generated macro for force_space_before (function)
macro_rules! Depcrate_macrosforce_space_before {
() => {
// Module: crate::macros
// Provides: {"force_space_before"}
// Dependencies: {}
fn force_space_before (tok : & TokenKind) -> bool { debug ! ("tok: force_space_before {:?}" , tok) ; match tok { TokenKind :: Eq | TokenKind :: Lt | TokenKind :: Le | TokenKind :: EqEq | TokenKind :: Ne | TokenKind :: Ge | TokenKind :: Gt | TokenKind :: AndAnd | TokenKind :: OrOr | TokenKind :: Bang | TokenKind :: Tilde | TokenKind :: PlusEq | TokenKind :: MinusEq | TokenKind :: StarEq | TokenKind :: SlashEq | TokenKind :: PercentEq | TokenKind :: CaretEq | TokenKind :: AndEq | TokenKind :: OrEq | TokenKind :: ShlEq | TokenKind :: ShrEq | TokenKind :: At | TokenKind :: RArrow | TokenKind :: LArrow | TokenKind :: FatArrow | TokenKind :: Plus | TokenKind :: Minus | TokenKind :: Star | TokenKind :: Slash | TokenKind :: Percent | TokenKind :: Caret | TokenKind :: And | TokenKind :: Or | TokenKind :: Shl | TokenKind :: Shr | TokenKind :: Pound | TokenKind :: Dollar => true , _ => false , } }
};
}
