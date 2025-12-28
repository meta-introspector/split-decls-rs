macro_rules! parse_expr_from_str {
    () => {
        pub fn parse_expr_from_str (s : & str , edition : Edition) -> Option < ast :: Expr > { let s = s . trim () ; let file = ast :: SourceFile :: parse (& format ! ("const _: () = ({s}\n);") , edition ,) ; let expr = file . syntax_node () . descendants () . find_map (ast :: ParenExpr :: cast) ? ; expr . expr () }
    };
}

parse_expr_from_str!();