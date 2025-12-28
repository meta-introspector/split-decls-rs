macro_rules! ClosureKind {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum ClosureKind { Fn , FnMut , FnOnce , }
    };
}

ClosureKind!();