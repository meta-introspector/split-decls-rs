macro_rules! make_req_fns {
    () => {
        macro_rules ! make_req_fns { ($ ($ name : ident , $ level : expr) ,+) => { $ (# [inline] pub fn $ name < A > (req : & http :: Request < A >) -> tracing :: Span { tracing :: span ! ($ level , "request" , method = ? req . method () , uri = ? req . uri () ,) }) + } }
    };
}

make_req_fns!()