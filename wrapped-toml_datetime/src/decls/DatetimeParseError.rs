macro_rules! deps {
    () => {
        Datetime!();
    };
}

macro_rules! DatetimeParseError {
    () => {
        deps!();
        # [doc = " Error returned from parsing a `Datetime` in the `FromStr` implementation."] # [derive (Debug , Clone)] # [non_exhaustive] pub struct DatetimeParseError { what : Option < & 'static str > , expected : Option < & 'static str > , }
    };
}

DatetimeParseError!()