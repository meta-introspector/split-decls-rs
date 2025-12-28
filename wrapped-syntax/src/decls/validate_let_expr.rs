macro_rules! deps {
    () => {
        SyntaxError!();
    };
}

macro_rules! validate_let_expr {
    () => {
        deps!();
        fn validate_let_expr (let_ : ast :: LetExpr , errors : & mut Vec < SyntaxError >) { let mut token = let_ . syntax () . clone () ; loop { token = match token . parent () { Some (it) => it , None => break , } ; if ast :: ParenExpr :: can_cast (token . kind ()) { continue ; } else if let Some (it) = ast :: BinExpr :: cast (token . clone ()) { if it . op_kind () == Some (ast :: BinaryOp :: LogicOp (ast :: LogicOp :: And)) { continue ; } } else if ast :: IfExpr :: can_cast (token . kind ()) || ast :: WhileExpr :: can_cast (token . kind ()) || ast :: MatchGuard :: can_cast (token . kind ()) { return ; } break ; } errors . push (SyntaxError :: new ("`let` expressions are not supported here" , let_ . syntax () . text_range () ,)) ; }
    };
}

validate_let_expr!();