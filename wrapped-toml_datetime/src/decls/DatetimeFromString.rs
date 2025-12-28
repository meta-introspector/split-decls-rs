macro_rules! deps {
    () => {
        Datetime!();
    };
}

macro_rules! DatetimeFromString {
    () => {
        deps!();
        # [cfg (feature = "serde")] pub (crate) struct DatetimeFromString { pub (crate) value : Datetime , }
    };
}

DatetimeFromString!();