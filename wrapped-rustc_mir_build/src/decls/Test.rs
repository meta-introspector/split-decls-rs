macro_rules! deps {
    () => {
        Candidate!();
        TestKind!();
    };
}

macro_rules! Test {
    () => {
        deps!();
        # [doc = " A test to perform to determine which [`Candidate`] matches a value."] # [doc = ""] # [doc = " [`Test`] is just the test to perform; it does not include the value"] # [doc = " to be tested."] # [derive (Debug)] pub (crate) struct Test < 'tcx > { span : Span , kind : TestKind < 'tcx > , }
    };
}

Test!()