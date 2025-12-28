macro_rules! deps {
    () => {
        IntegerRadix!();
    };
}

macro_rules! ScalarKind {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] pub enum ScalarKind { String , Boolean (bool) , DateTime , Float , Integer (IntegerRadix) , }
    };
}

ScalarKind!()