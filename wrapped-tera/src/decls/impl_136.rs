macro_rules! deps {
    () => {
        GetValue!();
        Result!();
        Error!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl GetValue for bool { fn get_value (val : & Value) -> Result < Self > { val . as_bool () . ok_or_else (| | Error :: msg (format ! ("expected bool got {}" , val))) } }
    };
}

impl_136!();