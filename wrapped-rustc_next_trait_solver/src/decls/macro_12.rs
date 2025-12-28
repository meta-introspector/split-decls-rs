macro_rules! deps {
    () => {
        IsFirstInputType!();
    };
}

macro_rules! macro_12 {
    () => {
        deps!();
        TrivialTypeTraversalImpls ! { IsFirstInputType , }
    };
}

macro_12!()