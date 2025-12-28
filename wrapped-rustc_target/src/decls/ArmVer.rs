macro_rules! ArmVer {
    () => {
        # [derive (Debug , PartialEq , Copy , Clone)] enum ArmVer { ThumbV8M , Other , }
    };
}

ArmVer!()