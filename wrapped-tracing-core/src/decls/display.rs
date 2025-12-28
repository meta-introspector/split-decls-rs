macro_rules! deps {
    () => {
        DisplayValue!();
        Value!();
    };
}

macro_rules! display {
    () => {
        deps!();
        # [doc = " Wraps a type implementing `fmt::Display` as a `Value` that can be"] # [doc = " recorded using its `Display` implementation."] pub fn display < T > (t : T) -> DisplayValue < T > where T : fmt :: Display , { DisplayValue (t) }
    };
}

display!();