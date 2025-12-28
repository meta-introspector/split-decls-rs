macro_rules! deps {
    () => {
        Scope!();
        DeclareLetBindings!();
    };
}

macro_rules! ThenElseArgs {
    () => {
        deps!();
        # [doc = " Arguments to [`Builder::then_else_break_inner`] that are usually forwarded"] # [doc = " to recursive invocations."] # [derive (Clone , Copy)] struct ThenElseArgs { # [doc = " Used as the temp scope for lowering `expr`. If absent (for match guards),"] # [doc = " `self.local_scope()` is used."] temp_scope_override : Option < region :: Scope > , variable_source_info : SourceInfo , # [doc = " Determines how bindings should be handled when lowering `let` expressions."] # [doc = ""] # [doc = " Forwarded to [`Builder::lower_let_expr`] when lowering [`ExprKind::Let`]."] declare_let_bindings : DeclareLetBindings , }
    };
}

ThenElseArgs!();