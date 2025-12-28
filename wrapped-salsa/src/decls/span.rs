macro_rules! span {
    () => {
        macro_rules ! span { ($ level : ident , $ ($ x : tt) *) => { { let span = { # [cold] # [inline (never)] || { :: tracing :: span ! (:: tracing :: Level ::$ level , $ ($ x) *) } } ; if :: tracing :: enabled ! (:: tracing :: Level ::$ level) { span () } else { :: tracing :: Span :: none () } } } ; }
    };
}

span!();