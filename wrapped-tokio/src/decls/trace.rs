macro_rules! trace {
    () => {
        mod trace { use std :: future :: Future ; use std :: pin :: Pin ; use std :: task :: { Context , Poll } ; cfg_taskdump ! { pub (crate) use crate :: runtime :: task :: trace :: trace_leaf ; } cfg_not_taskdump ! { # [inline (always)] # [allow (dead_code)] pub (crate) fn trace_leaf (_ : & mut std :: task :: Context <'_ >) -> std :: task :: Poll < () > { std :: task :: Poll :: Ready (()) } } # [cfg_attr (not (feature = "sync") , allow (dead_code))] pub (crate) fn async_trace_leaf () -> impl Future < Output = () > { struct Trace ; impl Future for Trace { type Output = () ; # [inline (always)] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < () > { trace_leaf (cx) } } Trace } }
    };
}

trace!()