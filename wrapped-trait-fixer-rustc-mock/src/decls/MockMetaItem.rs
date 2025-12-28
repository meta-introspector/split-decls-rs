macro_rules! MockMetaItem {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct MockMetaItem ;
    };
}

MockMetaItem!()