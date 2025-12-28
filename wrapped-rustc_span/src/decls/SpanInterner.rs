macro_rules! deps {
    () => {
        SpanData!();
    };
}

macro_rules! SpanInterner {
    () => {
        deps!();
        # [derive (Default)] pub (crate) struct SpanInterner { spans : FxIndexSet < SpanData > , }
    };
}

SpanInterner!()