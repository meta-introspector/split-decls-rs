macro_rules! MockAttribute {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct MockAttribute ;
    };
}

MockAttribute!()