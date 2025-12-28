macro_rules! deps {
    () => {
        ActualSpan!();
        SpanState!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl From < & SpanState > for ActualSpan { fn from (span_state : & SpanState) -> Self { Self :: new (span_state . id . clone () , Some (span_state . meta)) } }
    };
}

impl_69!()