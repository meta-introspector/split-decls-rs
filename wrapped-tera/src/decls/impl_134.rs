macro_rules! deps {
    () => {
        Error!();
        OrderedF64!();
        GetValue!();
        Result!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl GetValue for OrderedF64 { fn get_value (val : & Value) -> Result < Self > { let n = val . as_f64 () . ok_or_else (| | Error :: msg (format ! ("expected number got {}" , val))) ? ; Ok (OrderedF64 :: new (n)) } }
    };
}

impl_134!()