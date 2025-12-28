macro_rules! deps {
    () => {
        Name!();
        Test!();
        Error!();
    };
}

macro_rules! ExpandedTest {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct ExpandedTest { pub name : Name , pub test : Test , pub error : Option < Error > , is_from_glob : bool , }
    };
}

ExpandedTest!()