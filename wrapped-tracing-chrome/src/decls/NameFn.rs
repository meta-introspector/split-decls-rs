macro_rules! deps {
    () => {
        EventOrSpan!();
    };
}

macro_rules! NameFn {
    () => {
        deps!();
        type NameFn < S > = Box < dyn Fn (& EventOrSpan < '_ , '_ , S >) -> String + Send + Sync > ;
    };
}

NameFn!()