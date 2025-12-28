macro_rules! deps {
    () => {
        Runner!();
    };
}

macro_rules! TestCases {
    () => {
        deps!();
        # [derive (Debug)] pub struct TestCases { runner : RefCell < Runner > , }
    };
}

TestCases!();