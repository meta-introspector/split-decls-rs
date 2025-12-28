macro_rules! deps {
    () => {
        SpanData!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < Ctx : Copy > SpanData < Ctx > { pub fn eq_ignoring_ctx (self , other : Self) -> bool { self . anchor == other . anchor && self . range == other . range } }
    };
}

impl_78!()