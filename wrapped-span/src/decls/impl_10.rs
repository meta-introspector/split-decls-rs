macro_rules! deps {
    () => {
        SpanAnchor!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl fmt :: Debug for SpanAnchor { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("SpanAnchor") . field (& self . file_id) . field (& self . ast_id) . finish () } }
    };
}

impl_10!()