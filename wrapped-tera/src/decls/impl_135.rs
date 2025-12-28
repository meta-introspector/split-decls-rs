macro_rules! deps {
    () => {
        Result!();
        Error!();
        GetValue!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl GetValue for i64 { fn get_value (val : & Value) -> Result < Self > { val . as_i64 () . ok_or_else (| | Error :: msg (format ! ("expected number got {}" , val))) } }
    };
}

impl_135!()