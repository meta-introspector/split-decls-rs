macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! TestOutcome {
    () => {
        deps!();
        struct TestOutcome < O , E > { pub completed : Vec < O > , pub errors : Vec < Error < O , E > > , }
    };
}

TestOutcome!()