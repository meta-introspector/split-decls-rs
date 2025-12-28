macro_rules! deps {
    () => {
        IntoSpans!();
    };
}

macro_rules! Group {
    () => {
        deps!();
        # [doc (hidden)] # [allow (non_snake_case)] pub fn Group < S : IntoSpans < Span > > (span : S) -> Group { Group { span : span . into_spans () , } }
    };
}

Group!();