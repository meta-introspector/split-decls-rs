macro_rules! deps {
    () => {
        PassMode!();
        ABI!();
    };
}

macro_rules! ArgAbi {
    () => {
        deps!();
        # [doc = " Information about how to pass an argument to,"] # [doc = " or return a value from, a function, under some ABI."] # [derive (Clone , PartialEq , Eq , Hash , HashStable_Generic)] pub struct ArgAbi < 'a , Ty > { pub layout : TyAndLayout < 'a , Ty > , pub mode : PassMode , }
    };
}

ArgAbi!();