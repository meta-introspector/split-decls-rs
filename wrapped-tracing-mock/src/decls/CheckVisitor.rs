macro_rules! deps {
    () => {
        ExpectedFields!();
    };
}

macro_rules! CheckVisitor {
    () => {
        deps!();
        pub (crate) struct CheckVisitor < 'a > { expect : & 'a mut ExpectedFields , ctx : & 'a str , subscriber_name : & 'a str , }
    };
}

CheckVisitor!()