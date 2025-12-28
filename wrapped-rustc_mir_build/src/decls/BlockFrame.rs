macro_rules! BlockFrame {
    () => {
        # [derive (Debug , PartialEq , Eq)] enum BlockFrame { # [doc = " Evaluation is currently within a statement."] # [doc = ""] # [doc = " Examples include:"] # [doc = " 1. `EXPR;`"] # [doc = " 2. `let _ = EXPR;`"] # [doc = " 3. `let x = EXPR;`"] Statement { # [doc = " If true, then statement discards result from evaluating"] # [doc = " the expression (such as examples 1 and 2 above)."] ignores_expr_result : bool , } , # [doc = " Evaluation is currently within the tail expression of a block."] # [doc = ""] # [doc = " Example: `{ STMT_1; STMT_2; EXPR }`"] TailExpr { info : BlockTailInfo } , # [doc = " Generic mark meaning that the block occurred as a subexpression"] # [doc = " where the result might be used."] # [doc = ""] # [doc = " Examples: `foo(EXPR)`, `match EXPR { ... }`"] SubExpr , }
    };
}

BlockFrame!();