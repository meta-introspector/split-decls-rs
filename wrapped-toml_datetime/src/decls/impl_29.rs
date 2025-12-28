macro_rules! deps {
    () => {
        DatetimeParseError!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        # [cfg (all (not (feature = "std") , feature = "serde"))] impl serde_core :: de :: StdError for DatetimeParseError { }
    };
}

impl_29!()