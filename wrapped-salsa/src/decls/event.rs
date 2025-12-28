macro_rules! event {
    () => {
        macro_rules ! event { ($ level : ident , $ ($ x : tt) *) => { { let event = { # [cold] # [inline (never)] || { :: tracing :: event ! (:: tracing :: Level ::$ level , $ ($ x) *) } } ; if :: tracing :: enabled ! (:: tracing :: Level ::$ level) { event () ; } } } ; }
    };
}

event!()