macro_rules! deps {
    () => {
        Error!();
        Result!();
        GetValue!();
        ArrayLen!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl GetValue for ArrayLen { fn get_value (val : & Value) -> Result < Self > { let arr = val . as_array () . ok_or_else (| | Error :: msg (format ! ("expected array got {}" , val))) ? ; Ok (ArrayLen (arr . len ())) } }
    };
}

impl_138!();