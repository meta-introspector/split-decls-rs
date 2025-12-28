macro_rules! deps {
    () => {
        ActualSpan!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < 'a , S > From < & SpanRef < 'a , S > > for ActualSpan where S : LookupSpan < 'a > , { fn from (span_ref : & SpanRef < 'a , S >) -> Self { Self :: new (span_ref . id () , Some (span_ref . metadata ())) } }
    };
}

impl_84!()