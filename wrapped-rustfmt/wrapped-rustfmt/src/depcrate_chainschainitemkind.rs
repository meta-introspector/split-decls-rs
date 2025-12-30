// Generated macro for ChainItemKind (enum)
macro_rules! Depcrate_chainsChainItemKind {
() => {
// Module: crate::chains
// Provides: {"ChainItemKind"}
// Dependencies: {}
# [derive (Debug)] enum ChainItemKind { Parent { expr : ast :: Expr , parens : bool , } , MethodCall (ast :: PathSegment , Vec < ast :: GenericArg > , ThinVec < ptr :: P < ast :: Expr > > ,) , StructField (symbol :: Ident) , TupleField (symbol :: Ident , bool) , Await , Use , Yield , Comment (String , CommentPosition) , }
};
}
