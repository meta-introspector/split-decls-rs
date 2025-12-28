macro_rules! AdtKind {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Serialize)] pub enum AdtKind { Enum , Union , Struct , }
    };
}

AdtKind!();