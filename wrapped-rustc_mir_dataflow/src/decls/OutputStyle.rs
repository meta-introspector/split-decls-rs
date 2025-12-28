macro_rules! OutputStyle {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq)] enum OutputStyle { AfterOnly , BeforeAndAfter , }
    };
}

OutputStyle!()