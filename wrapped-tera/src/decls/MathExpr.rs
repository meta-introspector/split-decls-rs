macro_rules! deps {
    () => {
        MathOperator!();
        Expr!();
    };
}

macro_rules! MathExpr {
    () => {
        deps!();
        # [doc = " A mathematical expression"] # [derive (Clone , Debug , PartialEq)] pub struct MathExpr { # [doc = " The left hand side of the expression"] pub lhs : Box < Expr > , # [doc = " The right hand side of the expression"] pub rhs : Box < Expr > , # [doc = " The operator used"] pub operator : MathOperator , }
    };
}

MathExpr!();