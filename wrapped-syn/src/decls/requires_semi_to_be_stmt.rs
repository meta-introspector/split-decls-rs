macro_rules! requires_semi_to_be_stmt {
    () => {
        # [cfg (feature = "full")] pub (crate) fn requires_semi_to_be_stmt (expr : & Expr) -> bool { match expr { Expr :: Macro (expr) => ! expr . mac . delimiter . is_brace () , _ => requires_comma_to_be_match_arm (expr) , } }
    };
}

requires_semi_to_be_stmt!();