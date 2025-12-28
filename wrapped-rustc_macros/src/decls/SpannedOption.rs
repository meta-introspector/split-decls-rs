macro_rules! deps {
    () => {
        SetOnce!();
    };
}

macro_rules! SpannedOption {
    () => {
        deps!();
        # [doc = " An [`Option<T>`] that keeps track of the span that caused it to be set; used with [`SetOnce`]."] pub (super) type SpannedOption < T > = Option < (T , Span) > ;
    };
}

SpannedOption!()