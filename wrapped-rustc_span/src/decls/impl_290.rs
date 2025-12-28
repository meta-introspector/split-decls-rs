macro_rules! deps {
    () => {
        SpanData!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl fmt :: Debug for SpanData { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . span () , f) } }
    };
}

impl_290!()