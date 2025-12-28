macro_rules! DeclareLetBindings {
    () => {
        # [doc = " Should lowering a `let` expression also declare its bindings?"] # [doc = ""] # [doc = " Used by [`Builder::lower_let_expr`] when lowering [`ExprKind::Let`]."] # [derive (Clone , Copy)] pub (crate) enum DeclareLetBindings { # [doc = " Yes, declare `let` bindings as normal for `if` conditions."] Yes , # [doc = " No, don't declare `let` bindings, because the caller declares them"] # [doc = " separately due to special requirements."] # [doc = ""] # [doc = " Used for match guards and let-else."] No , # [doc = " Let expressions are not permitted in this context, so it is a bug to"] # [doc = " try to lower one (e.g inside lazy-boolean-or or boolean-not)."] LetNotPermitted , }
    };
}

DeclareLetBindings!()