macro_rules! deps {
    () => {
        LinePosition!();
    };
}

macro_rules! LineRange {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub struct LineRange { pub start : LinePosition , pub end : LinePosition , }
    };
}

LineRange!()