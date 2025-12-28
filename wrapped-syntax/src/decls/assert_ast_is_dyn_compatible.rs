macro_rules! deps {
    () => {
        AstNode!();
    };
}

macro_rules! assert_ast_is_dyn_compatible {
    () => {
        deps!();
        # [test] fn assert_ast_is_dyn_compatible () { fn _f (_ : & dyn AstNode , _ : & dyn HasName) { } }
    };
}

assert_ast_is_dyn_compatible!();