macro_rules! deps {
    () => {
        CapturedQuery!();
    };
}

macro_rules! Backtrace {
    () => {
        deps!();
        pub struct Backtrace (Box < [CapturedQuery] >) ;
    };
}

Backtrace!()