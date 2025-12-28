macro_rules! deps {
    () => {
        RenameRule!();
    };
}

macro_rules! RenameAllRules {
    () => {
        deps!();
        # [derive (Copy , Clone)] pub struct RenameAllRules { pub serialize : RenameRule , pub deserialize : RenameRule , }
    };
}

RenameAllRules!();