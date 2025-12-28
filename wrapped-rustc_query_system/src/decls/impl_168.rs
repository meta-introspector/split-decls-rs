macro_rules! deps {
    () => {
        QueryLatch!();
        QueryMap!();
        QueryStackFrame!();
        QueryJobId!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl QueryJobId { fn query < I : Clone > (self , map : & QueryMap < I >) -> QueryStackFrame < I > { map . get (& self) . unwrap () . query . clone () } fn span < I > (self , map : & QueryMap < I >) -> Span { map . get (& self) . unwrap () . job . span } fn parent < I > (self , map : & QueryMap < I >) -> Option < QueryJobId > { map . get (& self) . unwrap () . job . parent } fn latch < I > (self , map : & QueryMap < I >) -> Option < & QueryLatch < I > > { map . get (& self) . unwrap () . job . latch . as_ref () } }
    };
}

impl_168!()