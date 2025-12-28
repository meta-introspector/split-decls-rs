macro_rules! RangeEnd {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum RangeEnd { Included , Excluded , }
    };
}

RangeEnd!();