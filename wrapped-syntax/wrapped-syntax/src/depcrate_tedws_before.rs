// Generated macro for ws_before (function)
macro_rules! Depcrate_tedws_before {
() => {
// Module: crate::ted
// Provides: {"ws_before"}
// Dependencies: {}
fn ws_before (position : & Position , new : & SyntaxElement) -> Option < SyntaxToken > { let prev = match & position . repr { PositionRepr :: FirstChild (_) => return None , PositionRepr :: After (it) => it , } ; if prev . kind () == T ! ['{'] && new . kind () == SyntaxKind :: USE && let Some (item_list) = prev . parent () . and_then (ast :: ItemList :: cast) { let mut indent = IndentLevel :: from_element (& item_list . syntax () . clone () . into ()) ; indent . 0 += 1 ; return Some (make :: tokens :: whitespace (& format ! ("\n{indent}"))) ; } if prev . kind () == T ! ['{'] && ast :: Stmt :: can_cast (new . kind ()) && let Some (stmt_list) = prev . parent () . and_then (ast :: StmtList :: cast) { let mut indent = IndentLevel :: from_element (& stmt_list . syntax () . clone () . into ()) ; indent . 0 += 1 ; return Some (make :: tokens :: whitespace (& format ! ("\n{indent}"))) ; } ws_between (prev , new) }
};
}
