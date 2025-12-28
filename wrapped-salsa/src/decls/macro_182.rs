macro_rules! deps {
    () => {
        Value!();
        Configuration!();
    };
}

macro_rules! macro_182 {
    () => {
        deps!();
        intrusive_adapter ! (ValueAdapter < C > = UnsafeRef < Value < C >>: Value < C > { link : LinkedListLink } where C : Configuration) ;
    };
}

macro_182!();