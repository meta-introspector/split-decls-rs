macro_rules! deps {
    () => {
        ParseError!();
        ParseCtxt!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < 'a , 'tcx > ParseCtxt < 'a , 'tcx > { fn expr_error (& self , expr : ExprId , expected : & 'static str) -> ParseError { let expr = & self . thir [expr] ; ParseError { span : expr . span , item_description : format ! ("{:?}" , expr . kind) , expected : expected . to_string () , } } fn stmt_error (& self , stmt : StmtId , expected : & 'static str) -> ParseError { let stmt = & self . thir [stmt] ; let span = match stmt . kind { StmtKind :: Expr { expr , .. } => self . thir [expr] . span , StmtKind :: Let { span , .. } => span , } ; ParseError { span , item_description : format ! ("{:?}" , stmt . kind) , expected : expected . to_string () , } } }
    };
}

impl_50!();