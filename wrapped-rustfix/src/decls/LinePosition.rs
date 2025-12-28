macro_rules! LinePosition {
    () => {
        # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub struct LinePosition { pub line : usize , pub column : usize , }
    };
}

LinePosition!();