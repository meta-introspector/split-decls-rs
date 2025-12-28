macro_rules! deps {
    () => {
        PassMode!();
        Layout!();
        Ty!();
    };
}

macro_rules! ArgAbi {
    () => {
        deps!();
        # [doc = " Information about the ABI of a function's argument, or return value."] # [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub struct ArgAbi { pub ty : Ty , pub layout : Layout , pub mode : PassMode , }
    };
}

ArgAbi!();