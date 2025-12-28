macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! as_str {
    () => {
        deps!();
        pub fn as_str (value : & Value , _ : & HashMap < String , Value >) -> Result < Value > { let value = render_to_string (| | format ! ("as_str for value of kind {}" , value) , | w | value . render (w)) ? ; to_value (value) . map_err (Error :: json) }
    };
}

as_str!();