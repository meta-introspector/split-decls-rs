macro_rules! deps {
    () => {
        Result!();
        Error!();
        GetValue!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl GetValue for String { fn get_value (val : & Value) -> Result < Self > { let str : Result < & str > = val . as_str () . ok_or_else (| | Error :: msg (format ! ("expected string got {}" , val))) ; Ok (str ? . to_owned ()) } }
    };
}

impl_137!()