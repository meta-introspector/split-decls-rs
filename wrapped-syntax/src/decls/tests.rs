macro_rules! deps {
    () => {
        AstNode!();
        SyntaxAnnotation!();
        Position!();
        SyntaxEditor!();
        SyntaxElement!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use expect_test :: expect ; use crate :: { AstNode , ast :: { self , make , syntax_factory :: SyntaxFactory } , } ; use super :: * ; # [test] fn basic_usage () { let root = make :: match_arm (make :: wildcard_pat () . into () , None , make :: expr_tuple ([make :: expr_bin_op (make :: expr_literal ("2") . into () , ast :: BinaryOp :: ArithOp (ast :: ArithOp :: Add) , make :: expr_literal ("2") . into () ,) , make :: expr_literal ("true") . into () ,]) . into () ,) ; let to_wrap = root . syntax () . descendants () . find_map (ast :: TupleExpr :: cast) . unwrap () ; let to_replace = root . syntax () . descendants () . find_map (ast :: BinExpr :: cast) . unwrap () ; let mut editor = SyntaxEditor :: new (root . syntax () . clone ()) ; let make = SyntaxFactory :: with_mappings () ; let name = make :: name ("var_name") ; let name_ref = make :: name_ref ("var_name") . clone_for_update () ; let placeholder_snippet = SyntaxAnnotation :: default () ; editor . add_annotation (name . syntax () , placeholder_snippet) ; editor . add_annotation (name_ref . syntax () , placeholder_snippet) ; let new_block = make . block_expr ([make . let_stmt (make . ident_pat (false , false , name . clone ()) . into () , None , Some (to_replace . clone () . into ()) ,) . into ()] , Some (to_wrap . clone () . into ()) ,) ; editor . replace (to_replace . syntax () , name_ref . syntax ()) ; editor . replace (to_wrap . syntax () , new_block . syntax ()) ; editor . add_mappings (make . finish_with_mappings ()) ; let edit = editor . finish () ; let expect = expect ! [[r#"
            _ => {
                let var_name = 2 + 2;
                (var_name, true)
            },"#]] ; expect . assert_eq (& edit . new_root . to_string ()) ; assert_eq ! (edit . find_annotation (placeholder_snippet) . len () , 2) ; assert ! (edit . annotations . iter () . flat_map (| (_ , elements) | elements) . all (| element | element . ancestors () . any (| it | & it == edit . new_root ()))) } # [test] fn test_insert_independent () { let root = make :: block_expr ([make :: let_stmt (make :: ext :: simple_ident_pat (make :: name ("second")) . into () , None , Some (make :: expr_literal ("2") . into ()) ,) . into ()] , None ,) ; let second_let = root . syntax () . descendants () . find_map (ast :: LetStmt :: cast) . unwrap () ; let mut editor = SyntaxEditor :: new (root . syntax () . clone ()) ; let make = SyntaxFactory :: without_mappings () ; editor . insert (Position :: first_child_of (root . stmt_list () . unwrap () . syntax ()) , make . let_stmt (make :: ext :: simple_ident_pat (make :: name ("first")) . into () , None , Some (make :: expr_literal ("1") . into ()) ,) . syntax () ,) ; editor . insert (Position :: after (second_let . syntax ()) , make . let_stmt (make :: ext :: simple_ident_pat (make :: name ("third")) . into () , None , Some (make :: expr_literal ("3") . into ()) ,) . syntax () ,) ; let edit = editor . finish () ; let expect = expect ! [[r#"
            let first = 1;{
                let second = 2;let third = 3;
            }"#]] ; expect . assert_eq (& edit . new_root . to_string ()) ; } # [test] fn test_insert_dependent () { let root = make :: block_expr ([] , Some (make :: block_expr ([make :: let_stmt (make :: ext :: simple_ident_pat (make :: name ("second")) . into () , None , Some (make :: expr_literal ("2") . into ()) ,) . into ()] , None ,) . into () ,) ,) ; let inner_block = root . syntax () . descendants () . flat_map (ast :: BlockExpr :: cast) . nth (1) . unwrap () ; let second_let = root . syntax () . descendants () . find_map (ast :: LetStmt :: cast) . unwrap () ; let mut editor = SyntaxEditor :: new (root . syntax () . clone ()) ; let make = SyntaxFactory :: with_mappings () ; let new_block_expr = make . block_expr ([] , Some (ast :: Expr :: BlockExpr (inner_block . clone ()))) ; let first_let = make . let_stmt (make :: ext :: simple_ident_pat (make :: name ("first")) . into () , None , Some (make :: expr_literal ("1") . into ()) ,) ; let third_let = make . let_stmt (make :: ext :: simple_ident_pat (make :: name ("third")) . into () , None , Some (make :: expr_literal ("3") . into ()) ,) ; editor . insert (Position :: first_child_of (inner_block . stmt_list () . unwrap () . syntax ()) , first_let . syntax () ,) ; editor . insert (Position :: after (second_let . syntax ()) , third_let . syntax ()) ; editor . replace (inner_block . syntax () , new_block_expr . syntax ()) ; editor . add_mappings (make . finish_with_mappings ()) ; let edit = editor . finish () ; let expect = expect ! [[r#"
            {
                {
                let first = 1;{
                let second = 2;let third = 3;
            }
            }
            }"#]] ; expect . assert_eq (& edit . new_root . to_string ()) ; } # [test] fn test_replace_root_with_dependent () { let root = make :: block_expr ([make :: let_stmt (make :: ext :: simple_ident_pat (make :: name ("second")) . into () , None , Some (make :: expr_literal ("2") . into ()) ,) . into ()] , None ,) ; let inner_block = root . clone () ; let mut editor = SyntaxEditor :: new (root . syntax () . clone ()) ; let make = SyntaxFactory :: with_mappings () ; let new_block_expr = make . block_expr ([] , Some (ast :: Expr :: BlockExpr (inner_block . clone ()))) ; let first_let = make . let_stmt (make :: ext :: simple_ident_pat (make :: name ("first")) . into () , None , Some (make :: expr_literal ("1") . into ()) ,) ; editor . insert (Position :: first_child_of (inner_block . stmt_list () . unwrap () . syntax ()) , first_let . syntax () ,) ; editor . replace (inner_block . syntax () , new_block_expr . syntax ()) ; editor . add_mappings (make . finish_with_mappings ()) ; let edit = editor . finish () ; let expect = expect ! [[r#"
            {
                let first = 1;{
                let second = 2;
            }
            }"#]] ; expect . assert_eq (& edit . new_root . to_string ()) ; } # [test] fn test_replace_token_in_parent () { let parent_fn = make :: fn_ (None , None , make :: name ("it") , None , None , make :: param_list (None , []) , make :: block_expr ([] , Some (make :: ext :: expr_unit ())) , Some (make :: ret_type (make :: ty_unit ())) , false , false , false , false ,) ; let mut editor = SyntaxEditor :: new (parent_fn . syntax () . clone ()) ; if let Some (ret_ty) = parent_fn . ret_type () { editor . delete (ret_ty . syntax () . clone ()) ; if let Some (SyntaxElement :: Token (token)) = ret_ty . syntax () . next_sibling_or_token () && token . kind () . is_trivia () { editor . delete (token) ; } } if let Some (tail) = parent_fn . body () . unwrap () . tail_expr () { editor . delete (tail . syntax () . clone ()) ; } let edit = editor . finish () ; let expect = expect ! [["fn it() {\n    \n}"]] ; expect . assert_eq (& edit . new_root . to_string ()) ; } }
    };
}

tests!();