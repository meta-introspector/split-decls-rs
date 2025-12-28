macro_rules! deps {
    () => {
        StringConcat!();
        Expr!();
        MathExpr!();
        LogicExpr!();
        Test!();
        MacroCall!();
        FunctionCall!();
        In!();
    };
}

macro_rules! ExprVal {
    () => {
        deps!();
        # [doc = " An expression is the node found in variable block, kwargs and conditions."] # [derive (Clone , Debug , PartialEq)] # [allow (missing_docs)] pub enum ExprVal { String (String) , Int (i64) , Float (f64) , Bool (bool) , Ident (String) , Math (MathExpr) , Logic (LogicExpr) , Test (Test) , MacroCall (MacroCall) , FunctionCall (FunctionCall) , Array (Vec < Expr >) , StringConcat (StringConcat) , In (In) , }
    };
}

ExprVal!()